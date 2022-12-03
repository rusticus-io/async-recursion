
fn fibo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

use async_recursion::async_recursion;
#[async_recursion]
async fn async_fibo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => async_fibo(n - 1).await + async_fibo(n - 2).await,
    }
}

#[tokio::main]
async fn main() {
    let n = 10;
    println!("The fibonacci number of n = {} is {}.", n, fibo(n));
    println!("The fibonacci number of n = {} is {}.", n, async_fibo(n).await);
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

    #[tokio::test]
    async fn async_test() {
        assert_eq!(async_fibo(0).await, 0);
        assert_eq!(async_fibo(1).await, 1);
        assert_eq!(async_fibo(2).await, 1);
        assert_eq!(async_fibo(3).await, 2);
        assert_eq!(async_fibo(4).await, 3);
        assert_eq!(async_fibo(5).await, 5);
        assert_eq!(async_fibo(10).await, 55);
    }
}
