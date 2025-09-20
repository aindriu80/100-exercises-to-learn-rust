pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }

    let mut result: u32 = 1;
    let mut i: u32 = 1;

    // Keep multiplying until `1` exceeds `n`.
    while i <= n {
        result = result.saturating_mul(i);
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
