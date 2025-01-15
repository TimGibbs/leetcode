pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    let mut counter = 0;
    for i in 0..words.len() {
        for j in i + 1..words.len() {
            if words[j].starts_with(&words[i])
                && words[j].ends_with(&words[i])  {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(count_prefix_suffix_pairs(vec!["a".to_string(),"aba".to_string(),"ababa".to_string(),"aa".to_string()]), 4);
    }
    #[test]
    fn case2() {
        assert_eq!(count_prefix_suffix_pairs(vec!["pa".to_string(),"papa".to_string(),"ma".to_string(),"mama".to_string()]), 2);
    }
    #[test]
    fn case3() {
        assert_eq!(count_prefix_suffix_pairs(vec!["abab".to_string(),"ab".to_string()]), 0);
    }
}