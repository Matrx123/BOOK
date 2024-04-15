fn main() {
   //vecotrs are used to store same type elements
    let mut _v : Vec<i32> = Vec::new();
   println!("vector {:?}", _v);
   //using a macro
   let mut _v = vec![1,23,45];
    println!("vector using a macro {:?}", _v);
    //updating a vector
    let mut v1 = Vec::new();
    v1.push("string");
    println!("V1 {:?}", v1);
    //reding element of vector
    //via indexing or get method
    //indexing which is 0 based
    let v2 = vec![1,2,3,4,5];
    let third : &i32 = &v2[2];
    let second: &i32 = &v2[1];
    println!("third {:?} {:?}", third, second);
    //using get method, which returns an Option<&T> to be used with match
    let fourth : Option<&i32> = v2.get(3);
    match fourth {
        Some(fourth) => println!("Fourth {:?}", fourth),
        None => println!("There is no fourth element"),
    }
    //check of we find the index out of bounds
    // let does_not_exist = &v2[100]; //panic
    let _does_not_exist = v2.get(100);
    //borrowing rule
    let mut v4 = vec![];
    v4.push(6);
    v4.push(7);
    v4.push(8);

    //immutable reference due to borrowing rule
    let item = &v4[0];
    //mutable borrow
    v4.push(9);
    //panic due to borrow rule
    //println!("first {:?}", item);
    
    // iterating over vector
    let mut v5 = vec![1,2,3,4,5,6];
    //loop
    for i in &mut v5 {
        *i += 10;
    }

    println!("v5 {:?}", v5);
}
