fn fibonacci_number(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n.clone()
    } else {
        fibonacci_number(n - 1) + fibonacci_number(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibo_is_correct() {
        assert_eq!(fibonacci_number(3), 2)
    }
}
