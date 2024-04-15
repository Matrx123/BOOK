#[derive(Debug)]
struct Point<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

#[derive(Debug)]
struct PointSingle<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn main() {
    let r: &i32;
    //    {
    //     let x:i32 = 5;
    //     r = &x;
    //    }
    // Wont compile as the reference is dropped before even using it. AKA dangling reference
    // println!("{:?}", r);
    //can be fixed by
    let x: i32 = 5;
    r = &x;
    println!("{:?}", r);

    println!("======== Check for Longest String among two ========");

    let string1 = String::from("abcder");
    let string3 = String::from("this is vipin joshi");
    let result;

    {
        let string2 = String::from("vipin is an OG");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("Longest of two is {:?}", result);

    let result2 = longest_unassociated_fixed(string1.as_str(), string3.as_str());
    println!("longest of two unassociated {:?}", result2);

    println!("======== Check for Longest String End ========");

    println!("======== Lifetime for struct ========");

    //It will be a compile time error, as x and y doesn't satisfy the i's lifetime,
    //THis can be fixed by introducing multiple lifetime parameters.
    //also by moving y up in level with x.
    let x = 3;
    let r;

    {
        let y = 5;
        let i = Point {
            x: &x,
            y: &y,
        };
        r = i.x;
    }

    println!("Struct : {:?}", r);

    println!("======== Lifetime for struct End ========");
}

fn longest<'a>(str1: &'a str, str2: &str) -> &'a str {
    // if str1.len() > str2.len() {
    //    str1
    // } else {
    //    str2
    // }
    str1
}

//Note: The return lifetime should be associated to lifetimes of one of the param passed.
// fn longest_unassociated<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     let result = String::from("this is vipin joshi");
//     result.as_str()
// }

//Note: Fixed this by returing "owned type" as it transfers ownership.
fn longest_unassociated_fixed<'a>(str1: &'a str, str2: &'a str) -> String {
    String::from("This is vipin joshi")
}

//NOTE::: RULES which compiler follows to check for lifetimes
//
// Rule one is for input and other two are for output.
//
// 1- Each parameter gets its own lifetime.
// 2- If there is exactly one input param than its lifetime will be
//    assigned to all output lifetime parameters.
// 3- If there are multiple input param, but of them is a self or mutable self param,
//    than the lifetime of self or mut self will be assigned to all output lifetime parameters.
