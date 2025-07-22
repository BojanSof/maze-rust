use std::collections::HashMap;

use crate::maze::Maze;
use crate::priority_queue::PriorityQueue;
use crate::solvers::solver::Solver;

pub struct DijkstraSolver;

impl Solver for DijkstraSolver {
    fn solve(&self, maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let graph = maze.to_graph();

        let mut queue = PriorityQueue::new();
        let mut distances: HashMap<(usize, usize), (usize, (usize, usize))> = HashMap::new();

        distances.insert(maze.start, (0, maze.start));
        queue.push((0, maze.start));

        while let Some((current_distance, current_node)) = queue.pop() {
            if current_node == maze.end {
                let mut path = vec![current_node];
                let mut current = current_node;

                while current != maze.start {
                    if let Some(&(_, prev)) = distances.get(&current) {
                        path.push(prev);
                        current = prev;
                    } else {
                        break; // should never happen
                    }
                }
                path.reverse();
                return Some(path);
            }

            if current_distance
                > distances
                    .get(&current_node)
                    .unwrap_or(&(usize::MAX, (0, 0)))
                    .0
            {
                continue; // Skip if we found a better path already
            }

            for &(neighbor, weight) in graph.get(&current_node).unwrap_or(&vec![]) {
                let new_distance = current_distance + weight;
                if new_distance < distances.get(&neighbor).unwrap_or(&(usize::MAX, (0, 0))).0 {
                    distances.insert(neighbor, (new_distance, current_node));
                    queue.push((new_distance, neighbor));
                }
            }
        }
        None // No path found
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell;
    use crate::maze::Maze;
    use crate::solvers::solver::Solver;

    #[test]
    fn test_dijkstra_solver_finds_path() {
        let cells = vec![
            vec![Cell::Path, Cell::Wall, Cell::Path],
            vec![Cell::Path, Cell::Path, Cell::Path],
            vec![Cell::Wall, Cell::Path, Cell::Path],
        ];
        let start = (0, 0);
        let end = (2, 2);

        let maze = Maze { cells, start, end };
        let solver = DijkstraSolver;

        let path = solver.solve(&maze);
        assert!(path.is_some());

        let path = path.unwrap();
        assert_eq!(path.first().cloned(), Some(start));
        assert_eq!(path.last().cloned(), Some(end));

        for i in 0..path.len() - 1 {
            let (r1, c1) = path[i];
            let (r2, c2) = path[i + 1];
            let dr = (r1 as isize - r2 as isize).abs();
            let dc = (c1 as isize - c2 as isize).abs();
            assert_eq!(dr + dc, 1, "Steps must be adjacent");
            assert_ne!(maze.cells[r2][c2], Cell::Wall, "Step goes through wall");
        }
    }

    #[test]
    fn test_dijkstra_solver_no_path() {
        let cells = vec![
            vec![Cell::Path, Cell::Wall, Cell::Path],
            vec![Cell::Wall, Cell::Wall, Cell::Path],
            vec![Cell::Wall, Cell::Path, Cell::Path],
        ];
        let start = (0, 0);
        let end = (2, 2);

        let maze = Maze { cells, start, end };
        let solver = DijkstraSolver;

        let path = solver.solve(&maze);
        assert!(path.is_none(), "Expected no path due to walls");
    }
}
