pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
    let n = graph.len();
    let mut state = vec![0; n]; // 0 = not visited, 1 = visiting, 2 = safe
    let mut result = Vec::new();

    fn dfs(node: usize, graph: &Vec<Vec<i32>>, state: &mut Vec<i32>) -> bool {
        if state[node] != 0 {
            return state[node] == 2; // Return true if the node is safe
        }

        state[node] = 1; // Mark as visiting
        for &neighbor in &graph[node] {
            if state[neighbor as usize] == 2 {
                continue;
            }
            if state[neighbor as usize] == 1 || !dfs(neighbor as usize, graph, state) {
                return false; // Found a cycle or unsafe path
            }
        }

        state[node] = 2; // Mark as safe
        true
    }

    // Perform DFS for each node
    for i in 0..n {
        if dfs(i, &graph, &mut state) {
            result.push(i as i32);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1(){
        assert_eq!(eventual_safe_nodes(vec![vec![1,2],vec![2,3],vec![5],vec![0],vec![5],vec![],vec![]]),vec![2,4,5,6]);
    }

    #[test]
    fn case2(){
        assert_eq!(eventual_safe_nodes(vec![vec![1,2,3,4],vec![1,2],vec![3,4],vec![0,4],vec![]]),vec![4]);
    }
}