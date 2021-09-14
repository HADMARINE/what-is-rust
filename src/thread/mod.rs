use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
    time::Duration,
};

pub fn run() {
    mutex_counter();
}

fn first_thread() {
    let thread_handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    thread_handler.join().unwrap();
}

fn mpsc_thread() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hi");
        thread::sleep(Duration::from_secs(2));
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(2));
        tx.send(String::from("Hello")).unwrap();
    });

    for recieved in rx {
        println!("Got : {}", recieved);
    }
}

fn mutex_thread() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn mutex_counter() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            thread::sleep(Duration::from_secs(1));
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result : {}", *counter.lock().unwrap());
}
