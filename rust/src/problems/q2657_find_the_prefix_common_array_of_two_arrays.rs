pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    const MAX_VALUE: usize = 50;
    let mut ctr = 0;
    let mut ret = Vec::with_capacity(a.len());
    let mut seen = [false; MAX_VALUE + 1];

    for (ai, bi) in a.iter().zip(b.iter()) {
        if ai == bi {
            ctr += 1;
        }
        if seen[*ai as usize] {
            ctr += 1;
        }
        if seen[*bi as usize] {
            ctr += 1;
        }
        seen[*ai as usize] = true;
        seen[*bi as usize] = true;
        ret.push(ctr);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(find_the_prefix_common_array(vec![1,3,2,4], vec![3,1,2,4]), vec![0,2,3,4]);
    }
    #[test]
    fn case2() {
        assert_eq!(find_the_prefix_common_array(vec![2,3,1], vec![3,1,2]), vec![0,1,3]);
    }
}