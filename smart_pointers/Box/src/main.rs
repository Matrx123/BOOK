enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{ Cons, Nil };

fn main() {
    println!("Box smart pointer");

    let b = Box::new(5);
    println!("{:?}", b);

    let ll = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
