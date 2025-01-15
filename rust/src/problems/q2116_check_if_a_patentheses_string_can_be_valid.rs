pub fn can_be_valid(s: String, locked: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut min_balance = 0;
    let mut max_balance = 0;

    for (c, l) in s.chars().zip(locked.chars()) {
        match (c, l) {
            ('(', '1') => {
                min_balance += 1;
                max_balance += 1;
            }
            (')', '1') => {
                min_balance -= 1;
                max_balance -= 1;
            }
            (_, '0') => {
                min_balance -= 1;
                max_balance += 1;
            }
            _ => unreachable!(),
        }

        min_balance = min_balance.max(0);

        if max_balance < 0 {
            return false;
        }
    }

    min_balance == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(can_be_valid("))()))".to_string(), "010100".to_string()), true);
    }
    #[test]
    fn case2() {
        assert_eq!(can_be_valid("()()".to_string(), "0000".to_string()), true);
    }
    #[test]
    fn case3() {
        assert_eq!(can_be_valid(")".to_string(), "0".to_string()), false);
    }
}