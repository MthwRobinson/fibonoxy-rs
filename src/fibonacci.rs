pub fn fibonacci_number(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_number(n - 1) + fibonacci_number(n - 2),
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
