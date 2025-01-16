pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut xor1 = 0;
    let mut xor2 = 0;

    // If nums2.len() is odd, all elements of nums1 contribute to the result
    if nums2.len() % 2 == 1 {
        // XOR all elements of nums1
        for num in &nums1 {
            xor1 ^= num;
        }
    }

    // If nums1.len() is odd, all elements of nums2 contribute to the result
    if nums1.len() % 2 == 1 {
        // XOR all elements of nums2
        for num in &nums2 {
            xor2 ^= num;
        }
    }

    xor1 ^ xor2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(xor_all_nums(vec![2,1,3], vec![10,2,5,0]), 13);
    }
    #[test]
    fn case2() {
        assert_eq!(xor_all_nums(vec![1,2], vec![3,4]), 0);
    }
}