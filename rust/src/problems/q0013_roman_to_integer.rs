use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_values: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
        .iter()
        .cloned()
        .collect();

    let chars: Vec<char> = s.chars().collect();
    let mut total = 0i32;
    let mut prev_value = 0i32;

    for &c in chars.iter().rev() {
        if let Some(&value) = roman_values.get(&c) {
            if value < prev_value {
                total -= value; // Subtract if a smaller value precedes a larger value
            } else {
                total += value; // Add otherwise
            }
            prev_value = value;
        } else {
            panic!("Invalid Roman numeral character: {}", c);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }
    #[test]
    fn case2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }
    #[test]
    fn case3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}