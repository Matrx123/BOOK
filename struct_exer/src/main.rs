use std::io;

#[derive(Debug)]
struct AreaRectangle {
    length:u32,
    width: u32,
}

fn main() {
    let mut length = String::new();
    let mut width = String::new();
    
    println!("Enter length");
    io::stdin()
    .read_line(&mut length)
    .expect("Failed to read input");

    println!("Enter width");
    io::stdin()
    .read_line(&mut width)
    .expect("Failed to read input");

    let length: u32 = length.trim().parse().expect("Please Enter a valid Number!");
    let width: u32 = width.trim().parse().expect("Please enter a valid Number");

    println!("Area of rectange by simple I/O : {:?}", length*width);

    let rectangle = AreaRectangle { length, width };
    println!("Rectange is {:?}", rectangle);

    let area = calculate_area(rectangle);
    println!("Area of rectange by using Struct : {:?}", area);
}

fn calculate_area(rectangle: AreaRectangle) -> u32 {
    rectangle.length * rectangle.width
}