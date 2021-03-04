pub fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        return x;
    }
    return gcd(y, x % y);
}

#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd() {
        let x: i32 = 27;
        let y: i32 = 18;
        assert_eq!(9, gcd(x, y));
    }
}
