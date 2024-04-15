fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest_no = print_largest(&number_list);
    println!("largest number {:?}", largest_no);

    
    let second_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let largest_no = print_largest(&second_list);
    println!("largest number {:?}", largest_no);
}

fn print_largest(list: &[i32]) -> &i32 {
    let mut largest_no = &list[0];
    for number in list {
        if number > largest_no {
            largest_no = number;
        }
    }
    largest_no
}