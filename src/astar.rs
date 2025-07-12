use std::collections::HashMap;

use crate::maze::Maze;
use crate::priority_queue::PriorityQueue;
use crate::solver::Solver;

pub struct AstarSolver;

impl Solver for AstarSolver {
    fn solve(&self, maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let graph = maze.to_graph();

        let mut queue = PriorityQueue::new();
        let mut distances: HashMap<(usize, usize), (usize, (usize, usize))> = HashMap::new();

        distances.insert(maze.start, (0, maze.start));
        queue.push((heuristic(maze.start, maze.end), maze.start));

        while let Some((_, current_node)) = queue.pop() {
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
            let current_cost = distances
                .get(&current_node)
                .map(|x| x.0)
                .unwrap_or(usize::MAX);

            for &(neighbor, weight) in graph.get(&current_node).unwrap_or(&vec![]) {
                let new_cost = current_cost + weight;
                if new_cost < distances.get(&neighbor).map(|x| x.0).unwrap_or(usize::MAX) {
                    distances.insert(neighbor, (new_cost, current_node));

                    let priority = new_cost + heuristic(neighbor, maze.end);
                    queue.push((priority, neighbor));
                }
            }
        }
        None // No path found
    }
}

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell;
    use crate::maze::Maze;
    use crate::solver::Solver;

    #[test]
    fn test_astar_solver_finds_path() {
        let cells = vec![
            vec![Cell::Path, Cell::Wall, Cell::Path],
            vec![Cell::Path, Cell::Path, Cell::Path],
            vec![Cell::Wall, Cell::Path, Cell::Path],
        ];
        let start = (0, 0);
        let end = (2, 2);

        let maze = Maze { cells, start, end };
        let solver = AstarSolver;

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
    fn test_astar_solver_no_path() {
        let cells = vec![
            vec![Cell::Path, Cell::Wall, Cell::Path],
            vec![Cell::Wall, Cell::Wall, Cell::Path],
            vec![Cell::Wall, Cell::Path, Cell::Path],
        ];
        let start = (0, 0);
        let end = (2, 2);

        let maze = Maze { cells, start, end };
        let solver = AstarSolver;

        let path = solver.solve(&maze);
        assert!(path.is_none(), "Expected no path due to walls");
    }
}
