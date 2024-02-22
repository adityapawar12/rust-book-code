fn main() { // my_string_literal is not valid here, it’s not yet declared
    // string created using string literal.
    let my_string_literal: &str = "Hello, world!";  // my_string_literal is valid from this point forward

    // do stuff with my_string_literal
    println!("{}",my_string_literal);

    // string created using String keyword
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // Value swap of integers.
    let x = 5;
    let _y = x;
                              
    // String swap
    // String reference to the heap is stored in stack(like legnth, pointer and capacity)
    // When we swap a string value like this there are two possibilities one is shallow copy(s2 will only point towards the heap loaction of pointer)
    // and deep copy(heap data will be also recreated and s2 will point towards that pointer)
    // When we swap string between variables rust by default does a shallow copy.
    // But this would lead to errors when one of these variables goes out of scope right.
    // To solve that when performing shallow copy rust also invalidates the first variable from which we swapped the value in this case s1.
    // So after value swapping s1 cannot be used this is much more safe and memory efficient.
    let s1 = String::from("hello");
    let _s2 = s1;

    // To perform a deep copy instead of default behaviour we can use clone method.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function and so is no longer valid here

    // Throws error because s is invalidated.
    // println!("{}", s);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    let _s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);
                                    
    println!("The length of '{}' is {}.", s2, len);
} // this scope is now over, and my_string_literal is no longer valid
// Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}