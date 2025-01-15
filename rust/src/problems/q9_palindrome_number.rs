pub fn is_palindrome(x: i32) -> bool {
    if x < 0 { return false; }

    let mut num = x;
    let mut digits = Vec::new();

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    for i in 0..digits.len()/2 {
        if digits[i] != digits[digits.len().wrapping_sub(i+1)] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::problems::q9_palindrome_number::is_palindrome;

    #[test]
    fn case1() {
        assert_eq!(is_palindrome(121), true);
    }
    #[test]
    fn case2() {
        assert_eq!(is_palindrome(-121), false);
    }
    #[test]
    fn case3() {
        assert_eq!(is_palindrome(10), false);
    }
}