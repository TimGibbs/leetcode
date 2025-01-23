pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid[0].len();

    // Prefix sums for rows to quickly calculate the sum of any subarray
    let mut top_prefix = vec![0i64; n + 1];
    let mut bottom_prefix = vec![0i64; n + 1];

    for i in 0..n {
        top_prefix[i + 1] = top_prefix[i] + grid[0][i] as i64;
        bottom_prefix[i + 1] = bottom_prefix[i] + grid[1][i] as i64;
    }

    let mut result = i64::MAX;

    // Simulate the first robot's path
    for i in 0..n {
        let points_top = top_prefix[n] - top_prefix[i + 1] ; // Points left on the top row
        let points_bottom = bottom_prefix[i]; // Points collected on the bottom row

        // Max points the second robot can collect
        let second_robot_points = points_top.max(points_bottom) ;

        // Minimize the maximum points the second robot can collect
        result = result.min(second_robot_points);
    }

    result
}

#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(grid_game(vec![vec![2,5,4], vec![1,5,1]]),4);
    }
    #[test]
    fn case2() {
        assert_eq!(grid_game(vec![vec![3,3,1], vec![8,5,2]]),4);
    }
    #[test]
    fn case3() {
        assert_eq!(grid_game(vec![vec![1,3,1,15], vec![1,3,3,1]]),7);
    }
}