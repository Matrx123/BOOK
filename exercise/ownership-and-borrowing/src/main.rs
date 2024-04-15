fn main() {
    println!("Ownership and borrowing exercise");   

    let x = String::from("j j");
    let y = &x;
    println!("{}, {}", x, y);

    let s1 = String::from("v j");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    let s = give_ownership();
    println!("{}", s);

    let t = (1, 2, (), "hello".to_string());
    let u = t;
    println!("{:?}, {:?}", t, u);
}

fn give_ownership() -> String {
    let s = String::from("h j");
    // Convert String to Vec
    let _s = &s.clone().into_bytes();
    s
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}