fn main() {
    let s = String::from("Hello there!");
    take_ownership(s);
    // println!("original S: {}", s);

    let x=5;
    makes_copy(x);
    println!("X: {:?}", x);

    let s1= String::from("return ownership");
    let returned_s1 = takes_and_gives_back(s1);
    println!("returned S1: {:?}", returned_s1);

    let s2 = String::from("Hola!!");
    let (s, length) = get_a_touple(s2);
    println!("Returned Values string: {:?} and length: {:?} - ", s, length);
}

fn get_a_touple(str:String) -> (String, usize) {
    let length = str.len();
    (str, length)
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn take_ownership (str :String){
    println!("ownership called: {:?}", str);
}

fn makes_copy(num: i32) {
    println!("makes copy function {:?}", num);
}