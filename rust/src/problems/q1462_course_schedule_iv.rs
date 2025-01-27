pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let num_courses = num_courses as usize;

    // Step 1: Initialize the adjacency matrix
    let mut reachable = vec![vec![false; num_courses]; num_courses];

    // Step 2: Populate the adjacency matrix with prerequisites
    for prereq in prerequisites {
        if !prereq.is_empty() {
            reachable[prereq[0] as usize][prereq[1] as usize] = true;
        }
    }

    // Step 3: Compute transitive closure using Floyd-Warshall
    for k in 0..num_courses {
        for i in 0..num_courses {
            for j in 0..num_courses {
                reachable[i][j] |= reachable[i][k] && reachable[k][j];
            }
        }
    }

    // Step 4: Answer the queries
    queries
        .iter()
        .map(|query| reachable[query[0] as usize][query[1] as usize])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(check_if_prerequisite(2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]]),vec![false,true])
    }
    #[test]
    fn case2() {
        assert_eq!(check_if_prerequisite(2, vec![vec![]], vec![vec![1, 0], vec![0, 1]]),vec![false,false])
    }
    #[test]
    fn case3() {
        assert_eq!(check_if_prerequisite(3, vec![vec![1,2],vec![1,0],vec![2,0]], vec![vec![1, 0], vec![1,2]]),vec![true,true])
    }
}