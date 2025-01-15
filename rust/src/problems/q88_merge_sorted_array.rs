pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
    nums1.truncate(m as usize);

    let mut i = 0usize;
    let mut j = 0usize;
    while j < nums2.len() {

        while i < nums1.len() && nums1[i] < nums2[j] {
            i+=1;
        }
        nums1.insert(i, nums2[j]);
        i+=1;
        j+=1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }
    #[test]
    fn case2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }
    #[test]
    fn case3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }
}