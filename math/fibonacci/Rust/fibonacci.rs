// Rerturns the nth element of the fibonacci sequence 0,1,1,2,3.....
fn fibo(n :i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => (fibo(n-1) + fibo(n-2))
    }
}

fn main() {
    println!("{}",fibo(10));
    println!("{}",fibo(8));
    println!("{}",fibo(5));
    println!("{}",fibo(2));
    
}
