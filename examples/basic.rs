use loo::ThreadPool;

fn main() {
    let thread_pool = ThreadPool::new(4);
    
    thread_pool.submit(|| { let x = fib(50); println!("{}", x);});
    thread_pool.submit(|| { let x = fib(51); println!("{}", x);});
    thread_pool.submit(|| { let x = fib(52); println!("{}", x);});
    thread_pool.submit(|| { let x = fib(53); println!("{}", x);});
}

fn fib(n: usize) -> usize {
    if n <= 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}