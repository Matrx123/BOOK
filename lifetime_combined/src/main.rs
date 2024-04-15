use std::fmt::Display;

fn main() {
    let str1 = "abra ka dabra, gili gili appa";
    let str2 = "joshi";

    let result = longest(str1, str2, String::from("Fello Rustaceans!!"));
    println!("result {:?}", result);
}

fn longest<'a, T>(str1: &'a str, str2: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement <=> {:}", ann);

    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
