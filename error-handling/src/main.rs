use std::fs::File;
//use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    println!("Welcome to error handling class");

    //panic!("crash and burn");

    // let greeting_file = File::open("hello.txt");

    // let file = match greeting_file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error in creating the file! {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file!! {:?}", other_error);
    //         }
    //     }
    // };

    //unwrap
    // let _unwrap_file = File::open("hello.txt").unwrap();
    // let _expect_file = File::open("hello.txt").expect("File should be included");


}
