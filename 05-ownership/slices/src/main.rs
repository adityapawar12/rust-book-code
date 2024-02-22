fn main() {
    // let mut s = String::from("hello world");

    // let _word = first_word(&s); // word will get the value 5

    // s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // String slices in rust which create references to specific parts of the string.
    // let s = String::from("hello world");

    // let _hello = &s[0..5];
    // let _world = &s[6..11];

    // We can just enter last index if we want to start from index 0 and end index if we want to slice till the last index.
    // let s = String::from("hello");

    // let _slice = &s[0..2];
    // let _slice = &s[..2];
    // let _slice = &s[2..];

    // This will throw error because we cannot clear s strings value because there is a immutable reference to it's first word in word variable.
    // And we will need to create a mutable reference to it for using .clear method which we cannot do now.
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // error!

    // println!("the first word is: {}", word);

    // let my_string = String::from("hello world");

    // // `first_word` works on slices of `String`s, whether partial or whole
    // let _word = first_word(&my_string[0..6]);
    // let _word = first_word(&my_string[..]);
    // // `first_word` also works on references to `String`s, which are equivalent
    // // to whole slices of `String`s
    // let _word = first_word(&my_string);

    // let my_string_literal = "hello world";

    // // `first_word` works on slices of string literals, whether partial or whole
    // let _word = first_word(&my_string_literal[0..6]);
    // let _word = first_word(&my_string_literal[..]);

    // // Because string literals *are* string slices already,
    // // this works too, without the slice syntax!
    // let _word = first_word(my_string_literal);

    // Array slices
    // let a = [1, 2, 3, 4, 5];
    // let a = [1, 2, 3, 4, 5];

    // let slice = &a[1..3];

    // assert_eq!(slice, &[2, 3]);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}