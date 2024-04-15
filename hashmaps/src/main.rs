use std::collections::HashMap;
//its homogenious like vectors, doesn't have a builtin macro.
//HashMap<k,v>
//stores in heap

fn main() {
    println!("Hash maps");

    let mut scores = HashMap::new();

    //values will be overwritten if same key is inserted more than once
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);
    scores.insert(String::from("yellow"), 80);

    println!("hashmap : {:?}", scores);

    //get 

    let team_name = "blue";
    let score: Option<&i32> = scores.get(team_name);

    let second_score = scores.get("yellow").copied().unwrap_or(0);

    println!("second score is {:?}", second_score);

    match score {
        Some(score) => println!("score is {:?}", score),
        None => println!("No value is returned"),
    };

    //iteration

    for (key, value) in &scores {
        println!("{:?} => {:?}", key, value);
    }

    println!("hashmap as borrow : {:?}", scores);

    //adding a key iff key isn't present

    scores.entry(String::from("green")).or_insert(60);
    scores.entry(String::from("yellow")).or_insert(20);

    println!("after entry {:?}", scores);

    //updating a value in a HashMap, or_insert() returns a mutable refrence (&mut V) to the value of current key.
    let value = scores.entry(String::from("orange")).or_insert(0);
    //we can update the value like this:
    *value += 10;
    
   //excersise, check how many time a word has repeated.

   let word = "hello world wonderfull world";
    let mut hashmap = HashMap::new();

   for word in word.split_whitespace() {
        let count = hashmap.entry(word.to_string()).or_insert(0);
        *count += 1;
   }

   println!("the hashmap is {:?}", hashmap);

}
