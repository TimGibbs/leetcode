pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
    let mut ret = 0;
    let mut bitcount = num2.count_ones() as i32; // Number of set bits in num2

    // Traverse the bits of num1 from most to least significant
    for i in (0..32).rev() {
        // (1 << i) creates a mask where the i-th bit is set, eg i == 3 => 0b1000
        // num1 & mask  != 0 checks if the i-th bit in num1 is non zero
        if bitcount > 0 && (num1 & (1 << i)) != 0 {
            ret |= 1 << i; // Set this bit in x
            bitcount -= 1;
        }
    }

    // If there are still bits left to set, use the least significant unset bits
    for i in 0..32 {
        // (1 << i) creates moves the 1 i bits, eg i == 3 => 0b1000
        if bitcount > 0 && (ret & (1 << i)) == 0 {
            ret |= 1 << i; // Set this bit in x
            bitcount -= 1;
        }
    }

    ret

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(minimize_xor(3,5), 3);
    }
    #[test]
    fn case2() {
        assert_eq!(minimize_xor(1,12), 3);
    }
}

