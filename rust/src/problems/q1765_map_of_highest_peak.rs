use std::collections::VecDeque;
pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m = is_water.len();
    let n = is_water[0].len();
    let mut queue = VecDeque::new(); // Use VecDeque for FIFO queue

    // Initialize queue and set water cells' height to 0
    for i in 0..m {
        for j in 0..n {
            if is_water[i][j] == 1 {
                is_water[i][j] = 0;
                queue.push_back((i, j));
            } else {
                is_water[i][j] = -1; // Mark as unvisited
            }
        }
    }

    // Predefined movement directions
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // BFS
    while let Some((x, y)) = queue.pop_front() {
        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < m as i32 && ny < n as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if is_water[nx][ny] == -1 {
                    is_water[nx][ny] = is_water[x][y] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    is_water
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(highest_peak(vec![vec![0,1], vec![0,0]]), vec![vec![1,0],vec![2,1]]);
    }
    #[test]
    fn case2() {
        assert_eq!(highest_peak(vec![vec![0,0,1], vec![1,0,0], vec![0,0,0]]), vec![vec![1,1,0], vec![0,1,1], vec![1,2,2]])
    }
}