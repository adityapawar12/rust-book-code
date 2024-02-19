fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_arg_or_param(5);
    print_labeled_measurement(5, 'h');

    // statement
    let y = 6;
    // statements cannot be asssigned to something else.
    // let x = (let y = 6);

    // expressions 
    let ynew = {
        let xnew = 3;
        xnew + 1
    };
    println!("The value of ynew is: {ynew}");


    let fivevar = five();
    println!("The value of fivevar is: {fivevar}");

    let numplusone = plus_one(5);
    println!("The value of numplusone is: {numplusone}");

    // THIS IS HOW WE ADD COMMENTS IN RUST.
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_arg_or_param(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // returns value of x + 1 since it's an expression.
    x + 1

    // throws an error since x + 1; is a statement.
    // x + 1;
}