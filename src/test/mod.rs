#[cfg(test)]
mod tests {
    use crate::sub::add_two;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_bool() {
        assert!(true);
    }

    // #[test]
    fn panic_test() {
        panic!("Make this fail");
    }

    #[test]
    fn add_two_test() {
        assert_eq!(4, add_two(2));
    }
}
