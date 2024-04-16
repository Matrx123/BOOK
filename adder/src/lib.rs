struct Rectangle {
    width: u32,
    height: u32,
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 {
            panic!("Guess is less than 1 {}", value);
        } else if value > 100 {
            panic!("Guess is grater than 100 {}", value);
        }
        Guess { value }
    }
}

impl Rectangle {
    fn new(x: u32, y: u32) -> Self {
        Rectangle { width: x, height: y }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn add_two(a: i32) -> i32 {
    a + 2
}

fn add_three(a: i32) -> i32 {
    a + 3
}

fn greeting(name: &str) -> String {
    format!("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_holds() {
        let rectangle = Rectangle::new(12, 13);
        let other_rectangle = Rectangle::new(1, 3);
        let result = rectangle.can_hold(&other_rectangle);
        println!("Is it holds other rectange : {:}", result);
        assert_eq!(result, true);
    }
    #[test]
    fn check_add_two() {
        let result = add_two(2);
        assert_eq!(4, result);
    }
    #[test]
    fn check_add_three() {
        let result = add_three(2);
        assert_ne!(4, result);
    }
    #[test]
    fn check_string_contains() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting didn't contain name, the value was `{}`",
            result
        );
    }
    #[test]
    #[should_panic(expected = "Guess is grater than 100")]
    fn check_guess() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 5 { Ok(()) } else { Err(String::from("two plus two does not equal 4")) }
    }
}
