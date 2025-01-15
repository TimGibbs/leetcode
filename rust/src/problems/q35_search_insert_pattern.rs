pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut i = 0usize;
    while i < nums.len() && nums[i] < target {
        i+=1;
    }
    i as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }
    #[test]
    fn case2() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }
    #[test]
    fn case3() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}