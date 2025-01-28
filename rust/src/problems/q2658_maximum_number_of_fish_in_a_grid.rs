pub fn find_max_fish(grid: Vec<Vec<i32>>) -> i32 {
    let y_lim = grid.len();
    let x_lim = grid[0].len();

    let mut visited = vec![vec![false; x_lim]; y_lim];
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Recursive DFS to calculate the sum of connected fish
    fn dfs(x: usize, y: usize, grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>) -> i32 {
        if x >= grid[0].len() || y >= grid.len() || visited[y][x] || grid[y][x] == 0 {
            return 0;
        }

        visited[y][x] = true;
        let mut sum = grid[y][x];

        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 && (nx as usize) < grid[0].len() && (ny as usize) < grid.len() {
                sum += dfs(nx as usize, ny as usize, grid, visited);
            }
        }

        sum
    }

    let mut max_fish = 0;

    // Iterate through the grid and perform DFS for unvisited non-zero cells
    for y in 0..y_lim {
        for x in 0..x_lim {
            if !visited[y][x] && grid[y][x] > 0 {
                max_fish = max_fish.max(dfs(x, y, &grid, &mut visited));
            }
        }
    }

    max_fish
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            find_max_fish(vec![
                vec![0, 2, 1, 0],
                vec![4, 0, 0, 3],
                vec![1, 0, 0, 4],
                vec![0, 3, 2, 0]
            ]),
            7
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            find_max_fish(vec![
                vec![1, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 1]
            ]),
            1
        );
    }

    #[test]
    fn case3() {
        assert_eq!(find_max_fish(vec![vec![9, 10]]), 19);
    }

    #[test]
    fn case4() {
        assert_eq!(find_max_fish(vec![vec![8, 6], vec![2, 6]]), 22);
    }
}
