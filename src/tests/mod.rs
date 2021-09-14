#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_bool() {
        assert!(true);
    }

    #[test]
    #[should_panic]
    fn panic_test() {
        panic!("Make this fail");
    }

    #[test]
    fn add_two_test() {
        use crate::sub::add_two;

        assert_eq!(4, add_two(2));
    }

    #[test]
    fn assert_neq() {
        assert_ne!("a", "b", "Assertion failed, value : {}", "Hello world");
    }

    #[test]
    #[ignore = "It results error"]
    fn assert_neq_error() -> Result<(), String> {
        let value: bool = false;
        if value {
            Ok(())
        } else {
            Err(String::from("Value is false"))
        }
    }

    #[test]
    fn check_result() -> Result<(), String> {
        let value: bool = true;
        if value {
            Ok(())
        } else {
            Err(String::from("Value is false"))
        }
    }
}
