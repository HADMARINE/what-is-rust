use std::{
    ops::Add,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let m = Arc::new(Mutex::from(0));
    let mut hdlrs = vec![];

    for i in 1..10 {
        printer(i, String::from("ENTER LOOP"));
        let m = m.clone();

        let hdlr = thread::spawn(move || {
            printer(i, String::from("THREAD SPAWNED"));

            printer(i, String::from("WAITING FOR MUTEX"));
            let m_lock = match m.lock() {
                Ok(v) => v,
                Err(e) => panic!("Error"),
            };
            printer(i, String::from("MUTEX LOCKED"));
            printer(i, String::from("THREAD SLEEP START"));
            thread::sleep(Duration::from_secs(1));
            printer(i, String::from("THREAD SLEEP END"));

            m_lock.add(1);
            printer(i, String::from("INT ADDED"));
            ()
        });
        hdlrs.push(hdlr);
    }

    for i in hdlrs {
        i.join();
    }
}

fn printer(i: i32, s: String) {
    println!("[{}] : {}", i, s);
}
