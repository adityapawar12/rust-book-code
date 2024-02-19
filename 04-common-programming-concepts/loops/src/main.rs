fn main() {
    // infinite loop using loop keyword. use break or continue to stop it or ctrl c.
    // loop {
    //     println!("again!");
    // }

    // loop returning a value.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop with labels.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop in rust
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // While loop implementation using loop keyword.
    // loop {
    //     if number != 0 {
    //         println!("{number}!");

    //         number -= 1;
    //     } else {
    //         break
    //     }
    // }
    // println!("LIFTOFF!!!");

    // Traversing elements in an array using while loop. doing what for loop does using while keyword.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // for loop in rust.
    for element in a {
        println!("the value is: {element}");
    }

    // while loop implementation using for keyword.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}