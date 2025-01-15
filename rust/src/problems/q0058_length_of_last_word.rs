pub fn length_of_last_word(s: String) -> i32 {
    let mut word = false;
    let mut i = 0i32;
    for c in s.chars().rev(){
        if word && c== ' ' { return i;}
        if c!= ' ' {
            word = true;
            i+=1;
        }
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(length_of_last_word("hello world".to_string()), 5);
    }
    #[test]
    fn case2() {
        assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
    }
    #[test]
    fn case3() {
        assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
    }
}