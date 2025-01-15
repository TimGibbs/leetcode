pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words.iter().filter(|x|x.starts_with(&pref)).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(prefix_count(vec!["pay".to_string(), "attention".to_string(), "practice".to_string(), "attend".to_string()], "at".to_string()), 2)
    }
    #[test]
    fn case2() {
        assert_eq!(prefix_count(vec!["leetcode".to_string(), "win".to_string(), "loops".to_string(), "success".to_string()], "code".to_string()), 0)
    }
}