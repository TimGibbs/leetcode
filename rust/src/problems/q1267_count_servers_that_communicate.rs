pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut row_count = vec![0; m];
    let mut col_count = vec![0; n];
    
    for y in 0..m {
        for x in 0..n {
            if grid[y][x] >= 1 {
                row_count[y] +=1;
                col_count[x] +=1;
            }
        }
    }
    
    let mut total_communicating = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 && (row_count[i] > 1 || col_count[j] > 1) {
                total_communicating += 1;
            }
        }
    }

    total_communicating
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(count_servers(vec![vec![1,0],vec![0,1]]), 0);
    }
    #[test]
    fn case2() {
        assert_eq!(count_servers(vec![vec![1,0],vec![1,1]]), 3);
    }
    #[test]
    fn case3() {
        assert_eq!(count_servers(vec![vec![1,1,0,0], vec![0,0,1,0], vec![0,0,1,0], vec![0,0,0,1]]), 4);
    }
    
    
    
}