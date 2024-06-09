use std::rc::Rc;

//using Box, will result in moving value of a when used with , will result in moving value of
//a when used with b
//enum List {
//  Cons(i32, Box<List>),
//  Nill,
//}

enum List {
    Cons(i32, Rc<List>),
    Nill,
}

use List::{Cons, Nill};

fn main() {
    //creating a list with 5->10->Nill

    let a = Cons(5, Rc::new(Cons(10, Rc::new(Nill))));
    let b = Cons(3, Rc::new(a));
    let c = Cons(4, Rc::new(a));
}
