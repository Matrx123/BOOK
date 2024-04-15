fn main() {
    println!("Strign collection");

    //slice
    let slice = "vipin joshi";
    let _string = slice.to_string();

    //String::from("initial_value")
    let _single_slice = String::from("vipin joshi");

    //String::new()
    let mut empty_string = String::new();

    //takes slice
    empty_string.push_str("hello");
    println!("empty_string {:?}", empty_string);

    //takes character
    empty_string.push('!');
    println!("empty_string later {:?}", empty_string);

    //concatenation
    //using + operator

    let  s1 = String::from("vipin joshi");
    
    let s2 = String::from(", is a goof ball");
    
    let s = s1+&s2;

    println!("s ==> {:?}", s);

    //using format!() macro

    let s5 = String::from("hola!");

    let s6 = String::from("amigo");

    let s7 = format!("{s5} {s6}");

    println!("using format ==> {:?}", s7);

    //indexing in an string
    //Rust String doesn't support indexing

    let s8 = String::from("Hola! amigo");
    //println!("item {:?}", s8[0]);

    //string slices

    let hello = String::from("Здравствуйте");

    let slice = &hello[0..2];

    println!("slice {:?}", slice);

    //iterating over strings

    //chars
    for i in slice.chars() {
        println!("{:?}", i);
    }

    //bytes
    for i in slice.bytes() {
        println!("{:?}", i);
    }

}
