use std::fmt::Display;

fn main() {
    struct Pair<T>{
        x: T,
        y:T,
    }

    impl<T> Pair<T> {
        fn new(x:T, y:T) -> Self {
            Self{x, y}
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) -> String {
            if self.x >= self.y {
                format!("x is grater than y")
            } else {
                format!("y is grater than x")
            }
        }
    }

    let pair = Pair::new(100, 70);
    println!("{:?}", pair.cmp_display())

}
