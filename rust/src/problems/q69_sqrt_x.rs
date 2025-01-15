pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }

    let mut left = 1;
    let mut right = x/ 2;

    while left <= right {
        let mid = left + (right - left) / 2;
        let square = mid as usize * mid as usize;

        if square == x as usize {
            return mid;
        } else if square < x as usize {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    right
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(my_sqrt(4), 2);
    }
    #[test]
    fn case2() {
        assert_eq!(my_sqrt(8), 2);
    }
}