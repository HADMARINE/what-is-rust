mod messenger;
mod node;
mod sub;
mod tests;

use std::collections::HashMap;
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
    // let str: String = if__fn().to_string();
    // println!("{}", str);
    // let __s = String::from("HELLO_WORLD");
    // first_ptr_in_rs_fn(__s[1..3].to_string());
    // for_loop();
    // str_ownership_test_1()

    // println!("{}", first_tuples((1, 2)));

    // first_enum();

    // println!("{}", module_module::hello());
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", &s);
    // assert_eq!(1, 2);
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // print_option(five);
    // print_option(six);
    // print_option(none);

    // let mut map: HashMap<String, i32> = HashMap::new();

    // map.insert(String::from("Hello"), 12);
    // map.insert(String::from("World"), 22);

    // for (key, value) in &map {
    //     println!("{} : {}", key, value);
    // }
    // let g: GenericStruct<f32> = GenericStruct { x: 32.0, y: 32.0 };

    // let str_a = String::from("abc");
    // let str_b = String::from("abb");
    // let longer_str = return_longer_str(str_a.as_str(), str_b.as_str());

    // match longer_str {
    //     Ok(x) => println!("{}", x),
    //     Err(x) => println!("{}", x),
    // }

    // crate::sub::test_cons();
    crate::node::run();
}

fn return_longer_str<'str_lifetime>(
    x: &'str_lifetime str,
    y: &'str_lifetime str,
) -> Result<&'str_lifetime str, String> {
    let x_len = x.len();
    let y_len = y.len();

    if x_len > y_len {
        Ok(x)
    } else if x_len == y_len {
        let return_str = String::from(format!("Two string {} and {} are the same", x, y));
        Err(return_str)
    } else {
        Ok(y)
    }
}

struct GenericStruct<T> {
    x: T,
    y: T,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_option<T: std::fmt::Display>(x: Option<T>) {
    match x {
        None => println!("The value is None!"),
        Some(i) => println!("The value is {}!", i),
    }
}

fn change(some_str: &mut String) {
    some_str.push_str(", world");
}

fn first_word(s: &String) -> usize {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return 0usize;
}

mod module_module {
    pub fn hello() -> i32 {
        //can use like module_module::hello()
        hello_priv()
    }

    fn hello_priv() -> i32 {
        //cannot use outside of this module
        1
    }
}

fn first_enum() {
    #[derive(PartialEq)]
    enum IpAddrKind {
        // isize allowed for value
        V4 = 0,
        V6 = 1,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    //https://stackoverflow.com/questions/51429501/how-do-i-conditionally-check-if-an-enum-is-one-variant-or-another

    fn is_four_match(v: IpAddrKind) -> bool {
        match v {
            IpAddrKind::V4 => true,
            IpAddrKind::V6 => false,
        }
    }

    fn is_four_with_if(v: IpAddrKind) -> bool {
        if v == IpAddrKind::V4 {
            // derive PartialEq to validate like this
            true
        } else {
            false
        }
    }

    fn is_four_with_if_let(v: IpAddrKind) -> bool {
        if let IpAddrKind::V4 = v {
            true
        } else {
            false
        }
    }

    let is_four = is_four_with_if_let;

    println!("is 'four' ipv4? {}", is_four(four));
    println!("is 'six' ipv4? {}", is_four(six));
}

fn struct_impl() {
    struct Rect {
        w: u32,
        h: u32,
    }

    impl Rect {
        fn area(&self) -> u32 {
            self.w * self.h
        }

        fn square(size: u32) -> Rect {
            Rect { w: size, h: size }
        }
    }
    let rect1 = Rect { h: 30, w: 50 };

    println!("Area of the rect is {}", &rect1.area());

    let rect2 = Rect::square(1);
    println!("rect2: {}", rect2.area());
}

fn first_tuples(dim: (u32, u32)) -> u32 {
    let res = dim.0 * dim.1;
    println!("{}", &res);
    res
}

fn first_struct() {
    #[derive(Debug, Clone)] // allows to clone this struct can use like: #[derive(Clone, Debug, Copy, .....)]
    struct User {
        username: String,
        password: String,
        age: i8,
    }

    let user1 = User {
        age: 1,
        password: String::from("1234"),
        username: String::from("username"),
    };

    let mut user2 = User {
        age: 1,
        ..user1.clone() // this has to be the bottom of the declaration
    };

    user2.age = 2;

    fn show_user(u: &User) {
        println!(
            "username: {}, age: {}, password: {}",
            u.username, u.age, u.password
        );
    }

    show_user(&user1);
    show_user(&user2);

    println!("{:?}", &user1); // can print if only derived Debug
}

fn str_ownership_test_1() {
    let mut s1 = String::from("HELLO WORLD");

    println!("S1 : {}", s1);

    let s2 = &mut s1;
    let mut s3 = s2.clone();
    s3.push_str(" FROM YOUR BEST DEVELOPER");

    s2.push_str("!");
    println!("S2 : {}", s2);
    println!("S3 : {}", s3);
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5];

    for e in a.iter() {
        println!("{}", e);
    }
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

// fn spaces_fn() {
//     {
//         let spaces = "     ";
//         let spaces = spaces.len();
//     }

//     {
//         let mut spaces = "      ";
//         // spaces = spaces.len(); // error
//     }
// }

fn guess_fn() -> bool {
    let guess: bool = "true".parse().expect("Not a number!");
    guess
}
