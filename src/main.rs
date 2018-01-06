#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = Rectangle {
        width: width1,
        height: height1
    };
    
    println!("Area of rectangle is {} sq. pixels", area(&rect1));

    println!("rect1 is {:#?}", rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}