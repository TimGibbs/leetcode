pub fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(i) = haystack.find(&needle) { i as i32 } else { -1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }
    #[test]
    fn case2() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }
}