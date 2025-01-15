pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs[0].as_str();

    for s in &strs[1..] {
        while !s.starts_with(prefix) {
            prefix = &prefix[..prefix.len() - 1];
            if prefix.is_empty() {
                return String::new();
            }
        }
    }

    prefix.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(),"flow".to_string(),"flight".to_string()]), "fl".to_string());
    }
    #[test]
    fn case2() {
        assert_eq!(longest_common_prefix(vec!["dog".to_string(),"racecar".to_string(),"car".to_string()]), "".to_string());
    }
}