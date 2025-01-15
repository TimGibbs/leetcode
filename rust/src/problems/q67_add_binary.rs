pub fn add_binary(a: String, b: String) -> String {
    fn parts(a: Option<char>, b: Option<char>, carry: bool) -> (char, bool) {
        let a_val = a.unwrap_or('0').to_digit(2).unwrap();
        let b_val = b.unwrap_or('0').to_digit(2).unwrap();
        let sum = a_val + b_val + carry as u32;

        let result = if sum % 2 == 0 { '0' } else { '1' };
        let new_carry = sum > 1;

        (result, new_carry)
    }
    let mut response : Vec<char> = Vec::new();
    let mut carry = false;

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let mut i = a_chars.len();
    let mut j = b_chars.len();

    while i > 0 || j > 0 || carry {
        let a_char = if i > 0 { Some(a_chars[i - 1]) } else { None };
        let b_char = if j > 0 { Some(b_chars[j - 1]) } else { None };

        let (bit, new_carry) = parts(a_char, b_char, carry);

        response.push(bit);
        carry = new_carry;

        if i > 0 {
            i -= 1;
        }
        if j > 0 {
            j -= 1;
        }
    }
    response.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(add_binary("11".to_string(), "1".to_string()),"100")
    }
    #[test]
    fn case2() {
        assert_eq!(add_binary("1010".to_string(), "1011".to_string()),"10101")
    }
}