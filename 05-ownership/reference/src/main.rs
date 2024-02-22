fn main() {

    // At any given time, you can have either one mutable reference or any number of immutable references.
    // References must always be valid.


    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // When using a reference to the variable we cannot mutate or change the values of the reference itself
    // let s = String::from("hello");

    // change(&s);

    // passing in the parameter as a mutable value.
    // how i did it
    // let s = String::from("hello");
    // change_mutate(s);

    // How docs recommend it
    // let mut s = String::from("hello");
    // change_mutate(&mut s);

    // Two mutable references to one value are not allowed.
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // Having two mutable references using scopes
    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.

    // let r2 = &mut s;

    // We also cannot have a mutable reference while we have an immutable one to the same value.
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // // This is valid because the referece to the r1 and r2 end with the print line macro call.
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // // variables r1 and r2 will not be used after this point

    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    // dangling references\
    // let _reference_to_nothing = dangle();
    let _reference_to_nothing = undangle();
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn change_mutate(mut some_string: String) {
//     some_string.push_str(", world");
// }

// fn change_mutate(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. Danger!


fn undangle() -> String { // undangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s // we return the String, s
} // Here, s's ownership is returned by this function