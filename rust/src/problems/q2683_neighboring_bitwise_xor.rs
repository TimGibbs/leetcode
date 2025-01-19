pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    let mut i = 0;
    for d in derived.iter() {
        i ^= d;
    }
    i == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(does_valid_array_exist(vec![1, 1, 0]), true);
    }
    #[test]
    fn case2() {
        assert_eq!(does_valid_array_exist(vec![1, 1]), true);
    }
    #[test]
    fn case3() {
        assert_eq!(does_valid_array_exist(vec![1, 0]), false);
    }

}