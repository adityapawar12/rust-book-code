#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_tuple(rect1)
    );

    let rect_struct_1: Rectangle = Rectangle {
        width: 30,
        height: 30,
    }; 

    println!(
        "The area of the rectangle is {} square pixels.",
        area_using_struct(&rect_struct_1)
    );

    println!("rectangle struct is >>> {:#?}", rect_struct_1);

    let scale = 2;
    let rect_struct_2: Rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 30,
    };

    dbg!(&rect_struct_2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_using_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_using_struct(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
