pub fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut sorted = words.clone();
    sorted.sort_by(|a, b| a.len().cmp(&b.len()));
    let mut response : Vec<String> = Vec::with_capacity(words.len()-1);
    for (i, word) in sorted.iter().enumerate() {
        if sorted[i+1..].iter().any(|w| w.contains(word)) {
            response.push(word.clone());
        }
    }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(string_matching(vec!["mass".to_string(),"as".to_string(),"hero".to_string(),"superhero".to_string()]), vec!["as".to_string(), "hero".to_string()]);
    }
    #[test]
    fn case2() {
        assert_eq!(string_matching(vec!["leetcode".to_string(),"et".to_string(),"code".to_string()]), vec!["et".to_string(), "code".to_string()]);
    }
    #[test]
    fn case3() {
        assert_eq!(string_matching(vec!["blue".to_string(),"green".to_string(),"bu".to_string()]), Vec::<String>::new());
    }
}