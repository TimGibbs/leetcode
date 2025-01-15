pub fn minimum_length(s: String) -> i32 {
    fn count_chars(s: &str) -> [i32; 26] {
        let mut count = [0; 26];
        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }
        count
    }
    fn reduce_val(i:i32) -> i32 {
        let mut x = i;
        while x >= 3 {
            x -= 2;
        }
        x
    }

    let counts = count_chars(&s).iter().map(|&i|  reduce_val(i)).sum::<i32>();
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(minimum_length("abaacbcbb".to_string()), 5);
    }
    #[test]
    fn case2() {
        assert_eq!(minimum_length("aa".to_string()), 2);
    }
}