pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
    let m = mat.len();
    let n = mat[0].len();

    let mut positions = vec![(0, 0); m * n + 1];
    for i in 0..m {
        for j in 0..n {
            positions[mat[i][j] as usize] = (i, j);
        }
    }
    
    let mut row_counter = vec![0;m];
    let mut col_counter = vec![0;n];
    
    for (q,&v) in arr.iter().enumerate() {
        let (i, j) = positions[v as usize];

        row_counter[i] += 1;
        if row_counter[i] == n {
            return q as i32;
        }

        col_counter[j] += 1;
        if col_counter[j] == m {
            return q as i32;
        }
    }
    
    arr.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(first_complete_index(vec![1,3,4,2], vec![vec![1,4],vec![2,3]]),2);
    }

    #[test]
    fn case2() {
        assert_eq!(first_complete_index(vec![2,8,7,4,1,3,5,6,9], vec![vec![3,2,5],vec![1,4,6], vec![8,7,9]]),3);
    }
    
    #[test]
    fn case3() {
        assert_eq!(first_complete_index(vec![1,4,5,2,6,3], vec![vec![4,3,5],vec![1,2,6]]),1);
    }
}