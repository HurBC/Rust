fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

#[cfg(test)]
mod add_functions_test {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -3), 2);
        assert_eq!(add(1, 2), 3);
    }
    
    #[test]
    #[should_panic]
    fn add_error() {
        assert_eq!(add(1, 2), 5);
    }
    
    #[test]
    #[ignore = "I Don't care about negative numbers"]
    fn add_negatives() {
        assert_eq!(add(1, 2), 5);
    }
}

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::is_even;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(3));
        assert!(!is_even(5));
    }
}
