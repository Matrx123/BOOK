//Rust program to convert celsius to fahrenheit.
// F = C*9/5 + 32;
// C = (F - 32)*5/9;
//
use std::io;

fn main(){
    println!("Temperature conveter to Fahrenheit!!");
    println!("Enter celsius temprature to convert.");

    let mut degree_c = String::new();
    io::stdin()
        .read_line(&mut degree_c)
        .expect("Failed to read line");

    let degree_c:f32 = match degree_c.trim().parse() {
            Ok(num) => num,
            Err(_) => -1.0,
    };
    
    let fahrenheit:f32 = degree_c*9.0/5.0 + 32.0;
    println!("You entered {degree_c}C and in fahrenheit it equals to {fahrenheit}F!!");
}