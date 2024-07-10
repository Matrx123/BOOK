use std::thread;

fn main() {
    println!("Using `move` Closures wuth Threads!!");

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("here is a vector : {v:?}");
    });

    handle.join().unwrap();
}
