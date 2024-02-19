fn main() {
    let number = 5;

    // if conditional statement.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if condition needs to return a boolean value 
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    // else if block.
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = true;
    let conditionalnumber = if condition { 5 } else { 6 };

    // return type must be same.
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {conditionalnumber}");
}