
fn main() {
    println!("Raw pointer, unsafe block and unsafe function");

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;
    
    unsafe {
        println!("R1 : {:?}", r1);
        println!("R2 : {:?}", r2);
    }
}
