use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropped Custom pointer with data {:?}", self.data);
    }
}

fn main() {
    println!("Creating custom pointers");

    let c = CustomSmartPointer {
        data: String::from("first pointer"),
    };

    drop(c);

    let _d = CustomSmartPointer {
        data: String::from("second pointer"),
    };
}
