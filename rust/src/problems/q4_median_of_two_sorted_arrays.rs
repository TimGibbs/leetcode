pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
    let mut i = 0;
    let mut j = 0;

    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    while i < nums1.len() {
        merged.push(nums1[i]);
        i += 1;
    }

    while j < nums2.len() {
        merged.push(nums2[j]);
        j += 1;
    }

    let mid = merged.len() / 2;
    if merged.len() % 2 == 0 {
        (merged[mid - 1] as f64 + merged[mid] as f64) / 2.0
    } else {
        merged[mid] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
    }
    #[test]
    fn case2() {
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }

}