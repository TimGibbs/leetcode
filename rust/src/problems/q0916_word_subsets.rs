pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    fn count_chars(s: &str) -> [usize; 26] {
        let mut count = [0; 26];
        for ch in s.chars() {
            count[ch as usize - 'a' as usize] += 1;
        }
        count
    }

    let mut max_count = [0; 26];
    for word in &words2 {
        let count = count_chars(word);
        for i in 0..26 {
            max_count[i] = max_count[i].max(count[i]);
        }
    }

    words1
        .into_iter()
        .filter(|word| {
            let count = count_chars(word);
            (0..26).all(|i| count[i] >= max_count[i])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(word_subsets(vec!["amazon".to_string(),"apple".to_string(),"facebook".to_string(),"google".to_string(),"leetcode".to_string()], vec!["e".to_string(),"o".to_string()]), vec!["facebook","google","leetcode"]);
    }
    #[test]
    fn case2() {
        assert_eq!(word_subsets(vec!["amazon".to_string(),"apple".to_string(),"facebook".to_string(),"google".to_string(),"leetcode".to_string()], vec!["lc".to_string(),"eo".to_string()]), vec!["leetcode"]);
    }
    #[test]
    fn case3() {
        assert_eq!(word_subsets(vec!["acaac".to_string(),"cccbb".to_string(),"aacbb".to_string(),"caacc".to_string(),"bcbbb".to_string()], vec!["c".to_string(),"cc".to_string(),"b".to_string()]), vec!["cccbb"]);
    }
}