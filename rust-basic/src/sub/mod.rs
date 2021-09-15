use std::rc::Rc;
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

fn test_heap() -> Box<i32> {
    let b = Box::new(5);
    println!("b = {}", b);
    b
}

pub fn test_cons() {
    enum List<T> {
        Cons(T, Rc<List<T>>),
        Nil,
    }

    use List::{Cons, Nil};

    fn get_current_tree_value<'a, T>(tree: &Rc<List<T>>) -> Vec<&T> {
        let mut arr: Vec<&T> = Vec::new();
        let mut current_tree: &Rc<List<T>> = tree;
        loop {
            match &**current_tree {
                Cons(value, list) => {
                    arr.push(value.clone());
                    current_tree = &list;
                }
                Nil => break,
            }
        }
        println!("tree's ref count : {}", Rc::strong_count(&tree));

        arr
    }
    fn print_vec<T: std::fmt::Display>(vec: &Vec<T>, vec_name: String) {
        println!("___ Printing Vector : {} _____", vec_name);
        for item in vec {
            print!("{} ", &item);
        }
        println!("\n________");
    }

    let a = Rc::new(Cons(5, Rc::new(Nil)));
    let b = Rc::new(Cons(6, a.clone()));
    let c = Rc::new(Cons(7, b.clone()));

    print_vec(&get_current_tree_value(&c), String::from("c"));
    print_vec(&get_current_tree_value(&b), String::from("b"));
    print_vec(&get_current_tree_value(&a), String::from("a"));
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
        let heap_data = super::test_heap();
        assert_eq!(*heap_data, 5);
        println!("Created heap data, and it was success when evaluating value.");
        drop(heap_data);
        println!("Dropped heap data");
    }

    #[test]
    fn test_cons_test() {
        super::test_cons();
    }
}
