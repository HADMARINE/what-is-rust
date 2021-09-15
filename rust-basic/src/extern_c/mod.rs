#[link(name = "math", kind = "static")]

extern "C" {
    fn add(v1: f32, v2: f32) -> f32;
}

pub fn run() {
    println!("Hello from rust!");
    let res = unsafe { add(9., 12.) };
}
