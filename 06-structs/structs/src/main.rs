fn main() {
    // Defining a struct using struct keyword and adding the key value pairs in it(kind of like objects in js not sure if there are some serious differences).
    struct User {
        _active: bool,
        username: String,
        _email: String,
        _sign_in_count: u64,
    }

    // Creating an instance of a struct.
    let mut user1: User = User {
        _active: true,
        username: String::from("someusername123"),
        _email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };

    // accessing values in struct using dot notation.
    println!("User name is : {}", user1.username);

    // mutating values in struct using dot notation.
    user1.username = String::from("adi");
    println!("User name is : {}", user1.username);

    // A function that will take in name and email and build a User struct and return it.
    fn build_user(_email: String, username: String) -> User {
        // User {
        //     _active: true,
        //     username: username,
        //     _email: _email,
        //     _sign_in_count: 1,
        // }

        // field init shorthand because the username and email parameters have the same name as struct fields
        User {
            _active: true,
            username,
            _email,
            _sign_in_count: 1,
        }
    }

    let user2: User = build_user(String::from("adix@email.com"), String::from("Adix"));

    // user2 values printed.
    println!("User name is : {}", user2.username);

    // Creating a new User instance using one of the values from user1.
    // let _user3: User = User {
    //     _active: user1._active,
    //     username: user1.username,
    //     _email: String::from("another@example.com"),
    //     _sign_in_count: user1._sign_in_count,
    // };

    // Using struct update syntax to set a new email value for a User instance but to use the rest of the values from user1
    let _user4: User = User {
        _email: String::from("another@example.com"),
        ..user1
    };

    // Tuple struct. Structs which do not have key value pairs instead only have values.
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples,
    // and when naming each field as in a regular struct would be verbose or redundant.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Even though both types are made up of three i32 values they are still different Struct Tuples.
    let _black: Color = Color(0, 0, 0);
    let _origin: Point = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields. These structs are simillar to the () empty tuple.
    struct AlwaysEqual;
    let _subject: AlwaysEqual = AlwaysEqual;
}

// This code won't work because we will need to use lifetimes to make create a struct with reference data type like &str.
// struct User {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: "someusername123",
//         email: "someone@example.com",
//         sign_in_count: 1,
//     };
// }
