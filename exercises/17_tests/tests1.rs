// Tests are important to ensure that your code does what you think it should
// do.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
    println!("0 is even: {}", is_even(0));
    println!("1 is even: {}", is_even(1));
    println!("4 is even: {}", is_even(4));
    println!("7 is even: {}", is_even(7));
}

#[cfg(test)]
mod tests {
    // TODO: Import `is_even`. You can use a wildcard to import everything in
    // the outer module.
    use super::*;

    #[test]
    fn you_can_assert() {
        // TODO: Test the function `is_even` with some values.
        assert!(is_even(0));
        assert!(is_even(4));
        assert!(!is_even(1));
        assert!(!is_even(7));
    }
}
