// pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
//     nums.dedup();
//     nums.len() as i32
// }


pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut write_index = 1; // Index where the next unique element will be written

    for read_index in 1..nums.len() {
        if nums[read_index] != nums[read_index - 1] {
            nums[write_index] = nums[read_index];
            write_index += 1;
        }
    }

    nums.truncate(write_index); // Retain only the unique elements
    write_index as i32
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    }
    #[test]
    fn case2() {
        assert_eq!(remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]), 5);
    }
}