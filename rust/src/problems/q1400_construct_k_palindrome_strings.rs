pub fn can_construct(s: String, k: i32) -> bool {
    fn count_chars(s: &str) -> [i32; 26] {
        let mut count = [0; 26];
        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }
        count
    }

    let counts = count_chars(&s).iter().filter(|&i| i %2 == 1).count() as i32;
    counts <= k && s.len() as i32 >= k
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(can_construct("annabelle".to_string(), 2), true);
    }
    #[test]
    fn case2() {
        assert_eq!(can_construct("leetcode".to_string(), 3), false);
    }
    #[test]
    fn case3() {
        assert_eq!(can_construct("true".to_string(), 4), true);
    }
}