use std::io;
use std::collections::HashMap;

fn main() {
    println!("Collection exercise");
    println!("First problem");
    println!("Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.");
    println!("Solution::");

    let vec = vec![1, 1, 2, 3, 4, 5, 5, 5, 5];
    let (mode, median) = mode_median(&vec);
    println!("mode : {:?} and median : {:?}", mode, median);

    /////////////////////

    println!("Second problem");
    println!("Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!");
    println!("Solution::");

    let words = ["", "first", "apple", "banana", "orange", "eat", "omelette", "under"];
    for word in &words {
        println!("Pig Latin of {}: {}", word, pig_latin(word));
    }

    /////////////////////

    println!("Third problem");
    println!("Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.");
    println!("Solution::");

    text_interface();    

}

fn text_interface() {
    println!("welcome to Ecorp employee portal");

    let mut department_map: HashMap<String, Vec<String>>  = HashMap::new();


   loop {
    println!("Enter your department");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read input");
    let department = department.trim().to_string();

    println!("Enter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    let name = name.trim().to_string();

    let dept = department_map.entry(department).or_insert(Vec::new());
    dept.push(name);


    for (key, value) in &department_map {
        println!("{:?}", key);
        println!("  |");
        for item in value {
            println!("   --- {:?}", item);
        }
    }


   }
}


fn pig_latin(word : &str) -> String {
    let vowel_vec = vec!["a", "e", "i", "o", "u"];
    let mut updated_string = String::new();
    let mut consonent_suffix = String::new();

    for item in word.chars() {
        if vowel_vec.contains(&item.to_string().as_str()) {
            return format!("{}-hay", word);
        } else {
            updated_string.push_str(&word[1..]);
            consonent_suffix = format!("-{}ay", item);
            break;
        }
    }
    format!("{}{}", updated_string ,consonent_suffix)
}

fn mode_median(vec: &[i32]) -> (i32, i32) {
    let mut sorted_number = vec.to_vec();
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    
    sorted_number.sort();
    let mut mode = 0;
    let mut mode_val = 0;

    for i in &sorted_number {
        let entry = hashmap.entry(*i).or_insert(0);
        *entry += 1;
    }

    let length = sorted_number.len();
    let median = if length %2 == 0 {
        let mid = length/2;
        (sorted_number[mid-1] + sorted_number[mid]) / 2
    } else {
        sorted_number[length/2]
    };

    for (key, value) in &hashmap {
        if mode_val < *value {
            mode_val = *value;
            mode = *key;
        }
    }

    (mode, median)
}