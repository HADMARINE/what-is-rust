use std::{
    sync::{Arc, Mutex},
    vec,
};

fn main() {
    let arc_1 = Arc::new(Mutex::from(1));
    let arc_2 = Arc::new(Mutex::from(2));
    let mut v: Vec<Arc<Mutex<i32>>> = vec![arc_1, arc_2];
    let val = v.iter_mut().find(|x| *x.lock().unwrap() == 1);
    let found_val = match val {
        Some(v) => v,
        None => panic!("Fuck u"),
    };
    std::mem::replace(found_val, Arc::new(Mutex::from(3)));
    println!("found_val: {}", *v[0].lock().unwrap());
}
