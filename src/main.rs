
fn fibo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

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
