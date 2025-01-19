pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // right, left, down, up
    let direction_map = [1, 2, 3, 4]; // Match the input directions
    let m = grid.len();
    let n = grid[0].len();
    let mut cost = vec![vec![i32::MAX; n]; m];
    let mut deque = VecDeque::new();

    // Start at (0, 0) with cost 0
    cost[0][0] = 0;
    deque.push_front((0, 0, 0)); // (row, col, current_cost)

    while let Some((x, y, current_cost)) = deque.pop_front() {
        if current_cost > cost[x][y] {
            continue;
        }

        for (i, &(dx, dy)) in directions.iter().enumerate() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            // Ensure we are within grid bounds
            if nx < 0 || nx >= m as isize || ny < 0 || ny >= n as isize { continue; }
           
            let nx = nx as usize;
            let ny = ny as usize;

            // Calculate the cost to move to the new cell
            let move_cost = if direction_map[i] == grid[x][y] { 0 } else { 1 };

            // If the new cost is lower, update it and push to deque
            if current_cost + move_cost < cost[nx][ny] {
                cost[nx][ny] = current_cost + move_cost;
                if move_cost == 0 {
                    deque.push_front((nx, ny, cost[nx][ny])); // Prioritize cost 0 moves
                } else {
                    deque.push_back((nx, ny, cost[nx][ny])); // Delay for cost 1 moves
                }
            
            }
        }
    }

    // Return the cost to reach the bottom-right corner
    cost[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(min_cost(vec![vec![1,1,1,1], vec![2,2,2,2], vec![1,1,1,1], vec![2,2,2,2]]), 3);
    }
    #[test]
    fn case2() {
        assert_eq!(min_cost(vec![vec![1,1,3], vec![3,2,2], vec![1,1,4]]), 0);
    }
    #[test]
    fn case3() {
        assert_eq!(min_cost(vec![vec![1,2], vec![4,3]]), 1);
    }
}