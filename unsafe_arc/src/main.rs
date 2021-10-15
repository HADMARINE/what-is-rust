use std::{thread, sync::Arc};

struct TestStruct {
    a: i32,
};
fn main() {
    let arc_value = Arc::new(TestStruct{a:1});
    let mut arc_value_cloned_1 = arc_value.clone();
    let mut arc_value_cloned_2 = arc_value.clone();

    let hdlr_a = thread::spawn(move || {
            arc_value_cloned_1.a = 2;
    });
    
}
