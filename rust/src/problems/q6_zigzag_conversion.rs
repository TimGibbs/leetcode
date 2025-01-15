pub fn convert(s: String, num_rows: i32) -> String {
    if num_rows == 1 { return s; }
    let num_rows = num_rows as usize;
    let chars = s.chars().collect::<Vec<char>>();
    let mut response = String::with_capacity(s.len());
    let cad = 2 * num_rows - 2;
    for i in 0..num_rows {
        let mut position = i;
        while position < chars.len() {
            response.push(chars[position]);

            if i != 0 && i != num_rows - 1 {
                let diagonal = position + cad - 2 * i;
                if diagonal < chars.len() {
                    response.push(chars[diagonal]);
                }
            }

            position += cad;
        }
    }

    response
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR".to_string());
    }
    #[test]
    fn case2() {
        assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI".to_string());
    }
    #[test]
    fn case3() {
        assert_eq!(convert("A".to_string(), 1), "A".to_string());
    }
}