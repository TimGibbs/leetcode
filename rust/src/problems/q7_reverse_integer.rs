pub fn reverse(mut x: i32) -> i32 {
    let mut ret = 0i32;

    while x != 0 {
        let digit = x % 10;
        x /= 10;

        if let Some(new_ret) = ret.checked_mul(10).and_then(|r| r.checked_add(digit)) {
            ret = new_ret;
        } else {
            return 0;
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::reverse;
    #[test]
    fn case1() {
        assert_eq!(reverse(123), 321);
    }
    #[test]
    fn case2() {
        assert_eq!(reverse(-123), -321);
    }
    #[test]
    fn case3() {
        assert_eq!(reverse(120), 21);
    }
}