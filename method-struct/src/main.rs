use std::io;

#[derive(Debug)]
struct Rectangle {
    length:u32,
    width:u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_contain(&self, other_rectangle: &    Rectangle) -> bool {
        if self.length > other_rectangle.length && self.width > other_rectangle.width && self.area() > other_rectangle.area() {return true;}
        false
    }
}

fn main() {
    println!("Calcuate Area of rectangle");
    
    println!("Enter length");
    let mut length = String::new();
    io::stdin()
    .read_line(&mut length)
    .expect("Error in reading input");
    let length: u32 = length.trim().parse().expect("Enter a valid length");

    println!("Enter width");
    let mut width = String::new();
    io::stdin()
    .read_line(&mut width)
    .expect("Error in reading input");
    let width: u32 = width.trim().parse().expect("Enter a valid width");

    let rectangle1 = Rectangle {length, width};
    let rectangle2 = Rectangle {length: 10, width: 10};
    let rectangle3 = Rectangle {length:90, width:80};

    println!("Can reactangle-1 contains reactangle-2 : {:?} ", rectangle1.can_contain(&rectangle2));
    println!("Can reactangle-1 contains reactangle-3 : {:?} ", rectangle1.can_contain(&rectangle3));
    println!("Can reactangle-2 contains reactangle-3 : {:?} ", rectangle2.can_contain(&rectangle3));
    println!("Can reactangle-2 contains reactangle-1 : {:?} ", rectangle2.can_contain(&rectangle1));
    println!("Can reactangle-3 contains reactangle-1 : {:?} ", rectangle3.can_contain(&rectangle1));
    println!("Can reactangle-3 contains reactangle-2 : {:?} ", rectangle3.can_contain(&rectangle2));
}
