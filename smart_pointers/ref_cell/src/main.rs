//Interior mutability : mutating a value inside a immuatable value which appears immutable to outside world.

fn main() {
    println!("RefCell<T> interior mutability pattern!");
    let x = 5;
    let y = &mut x;

    let mut c = 10;
    let d = &c;

    *d = 20;
}
