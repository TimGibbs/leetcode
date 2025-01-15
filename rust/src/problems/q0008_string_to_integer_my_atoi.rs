pub fn my_atoi(s: String) -> i32 {
    let chars = s.trim_start().chars().collect::<Vec<char>>();
    if chars.len() == 0 { return 0i32;}
    let mut val = 0i32;
    let negative = chars[0] == '-';
    for i in 0..chars.len() {
        let ch = chars[i];
        if i == 0 && (ch == '-' || ch == '+') { continue; }
        if ch.is_ascii_digit() {
            if let Some(new_val) = val.checked_mul(10).and_then(|x| x.checked_add(ch.to_digit(10).unwrap() as i32)) {
                val = new_val
            } else {
                return if negative { i32::MIN } else { i32::MAX };
            }
        } else {
            return val * if negative { -1 } else { 1 };
        }
    }
    val * if negative { -1 } else { 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }
    #[test]
    fn case2() {
        assert_eq!(my_atoi("   -042".to_string()), -42);
    }
    #[test]
    fn case3() {
        assert_eq!(my_atoi("1337c0d3".to_string()), 1337);
    }
    #[test]
    fn case4() {
        assert_eq!(my_atoi("0-1".to_string()), 0);
    }
    #[test]
    fn case5() {
        assert_eq!(my_atoi("words and 987".to_string()), 0);
    }
}