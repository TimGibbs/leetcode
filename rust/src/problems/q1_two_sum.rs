use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = seen.get(&complement) {
            return vec![j as i32, i as i32];
        }
        seen.insert(num, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(two_sum(vec![2,7,11,15],9), vec![0,1]);
    }
    #[test]
    fn case2() {
        assert_eq!(two_sum(vec![3,2,4], 6), vec![1,2]);
    }
    #[test]
    fn case3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0,1]);
    }

}