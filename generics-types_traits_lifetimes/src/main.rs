use structs::Point;
use structs::Point2;
mod structs;


fn main() {
    let number_list = vec![1,14,11,90,7,100];
    let largest_no = get_largest(&number_list);
    println!("largest no is {:?}", largest_no);

    // let char_list = vec!['a','d','f','c','b'];
    // let largest_no = largest(&char_list);
    // println!("largest no is {:?}", largest_no);

    let p = Point {x: *largest_no, y: 10};
    let pf = Point {x: 1.2, y: 3.4};
    println!("p.x = {:?}, p.y = {:?}, {:?}", p.x(), p.y(), pf.distance_from_origin());

    let p2 = Point2 {x: 5, y: 10.4};
    let p3 = Point2 {x: "hello", y: 'v'};

    let p4 = p3.mixup(p2);
    println!("Mixup is {:?}", p4);

}

fn get_largest(list : &[i32])-> &i32 {
    let mut largest_no = &list[0];
    for i in list {
        if i > largest_no {
            largest_no = i;
        }
    }
    return largest_no;
}
//suppose we have a same method for char, we have re write the whole methid for chars.
//to avoid this we will use generics, so that it takes any list.


// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for i in list {
//         if i > largest {
//             largest = i;
//         }
//     }
//     largest
// }