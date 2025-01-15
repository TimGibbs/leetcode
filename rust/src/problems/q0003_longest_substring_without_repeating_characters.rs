use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map = HashMap::new();
    let mut output = 0;
    let mut left = 0;

    for (right, c) in s.chars().enumerate() {
        if let Some(&prev_index) = map.get(&c) {
            left = left.max(prev_index + 1);
        }
        map.insert(c, right);
        output = output.max(right - left + 1);
    }

    output as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }
    #[test]
    fn case2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }
    #[test]
    fn case3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }
}