
fn main() {
    println!("Hello, world!");
    let result = fib(5);
    println!("Result, {}", result);
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0
    }
    if n == 1 {
        return 1
    }

    let mut penultimate = 0;
    let mut last = 1;

    for _counter in (0..n-1).rev
    () {
        let new_number = penultimate + last;
        penultimate = last;
        last = new_number;
    };

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_0() {
        assert_eq!(fib(0), 0);
    }

    #[test]
    fn it_works_for_1() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn it_works_for_n() {
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
    }
}