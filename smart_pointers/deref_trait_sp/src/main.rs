use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    println!("using deref trait in smart pointres");

    let x = 5;
    // let y = &x;

    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //automatic Deref coercion
    let m = MyBox::new(String::from("Vipin Joshi"));
    //as MyBox is implemeting Deref so it returns &String and String also usees Deref so it gives &str
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello {:?}", name);
}
