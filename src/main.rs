use std::io;

fn main() {
    // println!("Hello, world!");

    // let guess = guess_provider();
    // if guess == "Hello" {
    //     println!("World");
    // } else {
    //     println!("{}", guess);
    // }
    // println!("{}", guess);
    // closure(1)(10);
    let str: String = if__fn().to_string();
    println!("{}", str);
    let __s = String::from("HELLO_WORLD");
    first_ptr_in_rs_fn(__s[1..3].to_string());
}

fn first_ptr_in_rs_fn(s: String) {
    println!("{}", s);
}

fn if__fn() -> i32 {
    if true {
        5
    } else {
        6
    }
}

fn scope_fn() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn array_index_fn() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}

fn tuples_fn() -> [(i32, f64, u8); 3] {
    let tup: [(i32, f64, u8); 3] = [(1, 2.0, 3), (1, 2.0, 3), (1, 2.0, 3)];
    tup
}

fn guess_provider() -> String {
    let mut str = String::new();
    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");
    str
}

fn range() {
    for n in 1..101 {
        println!("{}", n);
    }
}

fn closure(start: i32) -> impl Fn(i32) {
    let end_resolver = move |end: i32| {
        for n in start..end + 1 {
            println!("{}", n);
        }
    };
    end_resolver
}

fn ten_times<F>(f: F)
where
    F: Fn(i32),
{
    for index in 0..10 {
        f(index);
    }
}

fn spaces_fn() {
    {
        let spaces = "     ";
        let spaces = spaces.len();
    }

    {
        let mut spaces = "      ";
        // spaces = spaces.len(); // error
    }
}

fn guess_fn() -> bool {
    let guess: bool = "true".parse().expect("Not a number!");
    guess
}
