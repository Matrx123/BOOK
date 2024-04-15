use std::io;

fn main() {
    println!("Guess the index game with Arrays!!");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let arr:[u32; 5] = [12, 45, 76, 90, 81];
    let result:u32 =  arr[index];

    println!("Element is: {:?}", result);

}
