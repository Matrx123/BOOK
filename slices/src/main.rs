/*
write a function that takes a string of words separated by spaces
and returns the first word it finds in that string.
If the function doesn’t find a space in the string,
the whole string must be one word,
so the entire string should be returned.
*/

fn main() {
    let s = String::from("Hello world!!");
    let result = return_first_space_index(&s);

    println!("The resultant is: {:?}", result);

    let a=[1,2,3,4,5];
    let slice = &a[1..3];

    println!("Slice: {:?}", slice);
}

fn return_first_space_index (s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}