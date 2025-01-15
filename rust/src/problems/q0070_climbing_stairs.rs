use std::collections::HashMap;

pub fn climb_stairs(n: i32) -> i32 {
    fn climb_stairs_internal(n: i32, map : &mut HashMap<i32,i32>) -> i32 {
        if n < 2 { return 1 }
        if let Some(x) = map.get(&n) { return *x; }
        let a = climb_stairs_internal(n-1, map);
        let b = climb_stairs_internal(n-2, map);
        map.insert(n,a+b);
        a+b
    }
    let mut map = HashMap::new();
    climb_stairs_internal(n, &mut map)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(climb_stairs(2), 2);
    }
    #[test]
    fn case2() {
        assert_eq!(climb_stairs(3), 3);
    }
}