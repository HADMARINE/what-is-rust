use std::thread;
use std::time::Duration;

pub mod back_of_house {
    use std::convert::TryInto;

    fn hello() -> i32 {
        let add_one_v4 = |x: u32| x;

        add_one_v4(1).try_into().unwrap()
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn add_two_priv(a: i32) -> i32 {
    a + 2
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calcluating Slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn test_heap() -> i32 {
    let b = Box::new(5);
    println!("b = {}", b);
    b.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn add_two_priv() {
        assert_eq!(4, super::add_two_priv(2))
    }

    #[test]
    fn vector_eval() {
        let v1 = vec![1, 2, 3, 4];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), Some(&4));
        assert_eq!(v1_iter.next(), None);

        let map_result: Vec<_> = v1
            .iter()
            .map(|x| -> i32 { x + 1 })
            .filter(|x| *x == 2i32)
            .collect();

        let map_result_comp: Vec<i32> = vec![2, 3, 4, 5]
            .iter()
            .filter(|&x| *x == 2)
            .cloned()
            .collect();

        assert_eq!(map_result, map_result_comp);
    }

    #[test]
    fn test_heap_test() {
        assert_eq!(test_heap())
    }
}
