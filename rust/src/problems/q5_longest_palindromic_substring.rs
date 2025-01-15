pub fn longest_palindrome(s: String) -> String {
    fn is_palindrome(chars:&[char]) -> bool {
        for i in 0..chars.len()/2 {
            if chars[i] != chars[chars.len()-1-i] { return false; }
        }
        true
    }
    let chars = s.chars().collect::<Vec<char>>();

    for i in (0..chars.len()+1).rev(){
        for j in 0..chars.len() +1 - i{
            if is_palindrome(&chars[j..j+i]) {
                return chars[j..j+i].iter().copied().collect::<String>();
            }
        }
    }
    String::from("")
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1(){
        assert_eq!(longest_palindrome("babad".to_string()), "bab".to_string());
    }
    #[test]
    fn case2(){
        assert_eq!(longest_palindrome("cbbd".to_string()), "bb".to_string());
    }
}