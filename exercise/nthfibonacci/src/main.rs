use std::io;

fn main() {
    println!("Get your Nth number from Fibonacci series!!");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!!");

    let number:u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    //let tup:(String,String,String) = ("loop", "for", "while");
    //tup.1
    let mut n1:u32 = 0;
    let mut n2:u32 = 1;
    let mut item:u32 = 0;
    for _ in 0..number {
        item = n1+n2;
        n1=n2;
        n2=item;
    }
    println!("Item at pos {:?} is {:?} ", number, item);
}
