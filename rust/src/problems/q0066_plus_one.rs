pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut response = digits.clone();
    let mut carry = true;
    for i in (0..digits.len()).rev() {
        if !carry { break;}
        if digits[i]+1 == 10 {
            response[i] = 0
        } else {
            response[i] +=1;
            carry = false;
        }
    }
    if carry { response.insert(0, 1); }
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }
    #[test]
    fn case2() {
        assert_eq!(plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
    }
    #[test]
    fn case3() {
        assert_eq!(plus_one(vec![9]), vec![1,0]);
    }
}