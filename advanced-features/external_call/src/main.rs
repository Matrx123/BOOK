extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: i32 = 0;

fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    println!("Extern keyword usage for calling a external method");

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is {:?}", HELLO_WORLD);

    add_to_count(6);

    unsafe {
        println!("Counter is {}", COUNTER);
    }
}

#[no_mangle]

pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
