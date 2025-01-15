pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0usize;
    let mut r = height.len() - 1;
    let mut ret = 0i32;

    while l < r {
        let a = height[l].min(height[r])  * (r-l) as i32;
        ret = ret.max(a);
        if height [l] > height [r] {
            r-=1;
        } else {
            l+=1;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
    #[test]
    fn case2() {
    assert_eq!(max_area(vec![1,1]), 1);
    }
}
