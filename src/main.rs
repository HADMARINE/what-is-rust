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
    closure(1)(10);
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
