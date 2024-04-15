fn main() {
   let mut s1 = String::from("Hello World!!");
   let len = calculate_length(&s1);

   println!("length of string {:?} is {:?}", s1, len);

   change(&mut s1);

   let changed_len = calculate_length(&s1);

   println!("length of string {:?} is {:?}", s1, changed_len);

    let r1 = &mut s1;
    println!("{:?}", r1);
    let r2 = &mut s1;
    println!("{:?}", r2);
}

fn change(str: &mut String){
    str.push_str(" i am vipin");
}

fn calculate_length(str: &String) -> usize {
    str.len()
}
