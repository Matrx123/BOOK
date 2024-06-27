//using Box, will result in moving value of 'a' when used 'b' will result in moving value of
//a when used with 'c'
//enum List {
//  Cons(i32, Box<List>),
//  Nil,
//}

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    //creating a list with 5->10->Nill
    let a  = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    
    //Cloning an Rc<T> increases the reference Count

    let d = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating d = {}", Rc::strong_count(&d));

    let e = Cons(3, Rc::clone(&d));
    println!("Count after creating e = {}", Rc::strong_count(&d));

   { 
    let f = Cons(4, Rc::clone(&d));
    println!("Count after creating f = {}", Rc::strong_count(&d));
   }

   println!("Count after f goes out of scope = {}", Rc::strong_count(&d));
}
