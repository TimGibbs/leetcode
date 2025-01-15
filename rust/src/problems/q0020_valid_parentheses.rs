pub fn is_valid(s: String) -> bool {
    fn is_matching_pair(open: char, close: char) -> bool {
        match (open, close) {
            ('(', ')') => true,
            ('{', '}') => true,
            ('[', ']') => true,
            _ => false,
        }
    }

    let mut stack : Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' |'{' | '[' => stack.push(c),
            ')'|'}'|']' => {
                if let Some(top) = stack.pop() {
                    if !is_matching_pair(top, c) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(is_valid("()".to_string()), true);
    }
    #[test]
    fn case2() {
        assert_eq!(is_valid("()[]{}".to_string()), true);
    }
    #[test]
    fn case3() {
        assert_eq!(is_valid("(]".to_string()), false);
    }
    #[test]
    fn case4() {
        assert_eq!(is_valid("([])".to_string()), true);
    }

}