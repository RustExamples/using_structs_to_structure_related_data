fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (width1, height1);
    
    println!("Area of rectangle is {} sq. pixels", area(rect1));
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}