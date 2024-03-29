#[cfg(test)]
mod tests {
    // Basic test
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    use super::*;

    // Test that will pass because true
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    // Test that will pass because false and !
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    // Test that will pass because equal
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Test that will pass and display message
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }

    // Test that will pass because should panic with message
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Test that will pass because expect Ok
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // Test that will be ignore
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }

    // Test that will pass with a private function
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)

    // Will fail
    //String::from("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
            value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
            value);
        }

        Guess {
            value
        }
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
