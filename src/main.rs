
fn fibo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

async fn async_fibo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => async_fibo(n - 1).await + async_fibo(n - 2).await,
    }
}
/*
error[E0733]: recursion in an `async fn` requires boxing
  --> src/main.rs:10:32
   |
10 | async fn async_fibo(n: u64) -> u64 {
   |                                ^^^ recursive `async fn`
   |
   = note: a recursive `async fn` must be rewritten to return a boxed `dyn Future`
   = note: consider using the `async_recursion` crate: https://crates.io/crates/async_recursion

For more information about this error, try `rustc --explain E0733`.
error: could not compile `fibonacci` due to previous error
 */

fn calculate_sync(n: u64) {
    println!("The fibonacci number of n = {} is {}.", n, fibo(n));
}

fn main() {
    calculate_sync(10);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn sync_test() {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(3), 2);
        assert_eq!(fibo(4), 3);
        assert_eq!(fibo(5), 5);
        assert_eq!(fibo(10), 55);
    }
}
