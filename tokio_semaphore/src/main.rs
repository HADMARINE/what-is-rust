use tokio::sync::Semaphore;

fn main() {
    let semaphore = Semaphore::new(3);
    
    println!("Hello, world!");
}
