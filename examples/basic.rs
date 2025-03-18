use loo::ThreadPool;

fn main() {
    let pool = ThreadPool::new(1);
    
    pool.submit(move || {
        println!("Hello, world!");
    });
    
    let x: i32 = 2;
    pool.submit(move || {
        println!("Hello, world!");
    });
}