// use std::{
//     io::{BufRead, Write},
//     net::TcpListener,
// };

// fn main() {
//     let listener: TcpListener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();

//     for mut stream in listener.incoming().flatten() {
//         let mut rdr = std::io::BufReader::new(&mut stream);

//         let mut l = String::new();
//         rdr.read_line(&mut l).unwrap();
//         match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
//             ["GET", resource, "HTTP/1.1"] => {
//                 loop {
//                     let mut l = String::new();
//                     rdr.read_line(&mut l).unwrap();
//                     if l.trim().is_empty() {
//                         break;
//                     }
//                     println!("{l}");
//                 }
//                 let mut p = std::path::PathBuf::new();
//                 p.push("htdocs");
//                 p.push(resource.trim_start_matches("/"));
//                 if resource.ends_with("/") {
//                     p.push("index.html");
//                 }
//                 stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
//                 stream.write_all(&std::fs::read(p).unwrap()).unwrap();
//                 // stream
//                 //     .write_all(b"HTTP/1.1 200 OK\r\n\r\nHello, Thank you computerphile!")
//                 //     .unwrap();
//             }
//             _ => todo!(),
//         }
//     }
// }

use std::{
    io::{BufRead, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

fn handle_client(mut stream: TcpStream) {
    let mut reader = std::io::BufReader::new(&mut stream);

    // Read the request line
    let mut request_line = String::new();
    if let Err(err) = reader.read_line(&mut request_line) {
        eprintln!("Error reading request line: {}", err);
        return;
    }

    // Parse the request method and resource
    let mut parts = request_line.trim().split_whitespace();
    let method = parts.next().unwrap_or("");
    let resource = parts.next().unwrap_or("");

    // We only support GET method in this example
    if method != "GET" {
        eprintln!("Unsupported HTTP method: {}", method);
        return;
    }

    // Skip additional headers
    loop {
        let mut header = String::new();
        if let Err(err) = reader.read_line(&mut header) {
            eprintln!("Error reading header: {}", err);
            return;
        }
        if header.trim().is_empty() {
            break;
        }
    }

    // Construct the file path
    let mut file_path = PathBuf::from("src/htdocs");
    file_path.push(resource.trim_start_matches('/'));
    if file_path.is_dir() {
        file_path.push("index.html");
    }

    // Read file contents and send response
    match std::fs::read(&file_path) {
        Ok(content) => {
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                content.len()
            );
            if let Err(err) = stream.write_all(response.as_bytes()) {
                eprintln!("Error writing response: {}", err);
                return;
            }
            if let Err(err) = stream.write_all(&content) {
                eprintln!("Error writing file content: {}", err);
                return;
            }
        }
        Err(err) => {
            eprintln!("Error reading file {:?}: {}", file_path, err);
            let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
            if let Err(err) = stream.write_all(response.as_bytes()) {
                eprintln!("Error writing 404 response: {}", err);
                return;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(err) => {
                eprintln!("Error accepting connection: {}", err);
            }
        }
    }
}
