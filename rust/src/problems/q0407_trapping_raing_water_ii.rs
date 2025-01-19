pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let m = height_map.len();
    if m == 0 {
        return 0;
    }
    let n = height_map[0].len();
    if n == 0 {
        return 0;
    }

    // Directions for neighbors: right, left, down, up
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Priority queue (min-heap)
    let mut heap = BinaryHeap::new();
    let mut visited = vec![vec![false; n]; m];

    // Add all boundary cells to the heap
    for i in 0..m {
        for j in [0, n - 1] {
            heap.push(Reverse((height_map[i][j], i, j)));
            visited[i][j] = true;
        }
    }
    for j in 0..n {
        for i in [0, m - 1] {
            if !visited[i][j] {
                heap.push(Reverse((height_map[i][j], i, j)));
                visited[i][j] = true;
            }
        }
    }

    let mut water_trapped = 0;

    // Process cells in the heap
    while let Some(Reverse((height, x, y))) = heap.pop() {
        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            //skip if oob
            if nx < 0 || nx >= m as isize || ny < 0 || ny >= n as isize { continue; }
            
            let nx = nx as usize;
            let ny = ny as usize;

            if visited[nx][ny] { continue; }
            
            visited[nx][ny] = true;
            // Calculate the trapped water
            water_trapped += 0.max(height - height_map[nx][ny]);
            // Add the neighbor to the heap with the updated height
            heap.push(Reverse((height.max(height_map[nx][ny]), nx, ny)));
        }
    }

    water_trapped
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        assert_eq!(trap_rain_water(vec![vec![1,4,3,1,3,2],vec![3,2,1,3,2,4],vec![2,3,3,2,3,1]]), 4);
    }
    #[test]
    fn case2() {
        assert_eq!(trap_rain_water(vec![vec![3,3,3,3,3],vec![3,2,2,2,3],vec![3,2,1,2,3],vec![3,2,2,2,3],vec![3,3,3,3,3]]), 10);
    }
}