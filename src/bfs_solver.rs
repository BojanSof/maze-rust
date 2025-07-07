use std::collections::{HashMap, HashSet};

use crate::cell::Cell;
use crate::maze::Maze;
use crate::queue::Queue;
use crate::solver::Solver;

pub struct BfsSolver;

fn get_neighbors(row: usize, col: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut neighbors = Vec::new();

    for (dy, dx) in DIRS.iter() {
        let new_row = row as isize + dy;
        let new_col = col as isize + dx;

        if new_row >= 0 && new_row < height as isize && new_col >= 0 && new_col < width as isize {
            neighbors.push((new_row as usize, new_col as usize));
        }
    }

    neighbors
}

impl Solver for BfsSolver {
    fn solve(&self, maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let mut visited = HashSet::new();
        let mut queue = Queue::new();
        let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        queue.enqueue(maze.start);

        while let Some((row, col)) = queue.dequeue() {
            visited.insert((row, col));
            if (row, col) == maze.end {
                let mut path = vec![(row, col)];
                let mut current = (row, col);

                while let Some(&prev) = parent.get(&current) {
                    path.push(prev);
                    current = prev;
                }

                path.reverse();
                return Some(path);
            } else if maze.cells[row][col] != Cell::Wall {
                let neighbors = get_neighbors(row, col, maze.cells.len(), maze.cells[0].len());
                for (n_row, n_col) in neighbors {
                    if maze.cells[n_row][n_col] != Cell::Wall && !visited.contains(&(n_row, n_col))
                    {
                        parent.insert((n_row, n_col), (row, col));
                        queue.enqueue((n_row, n_col));
                    }
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
    use crate::solver::Solver;

    #[test]
    fn test_bfs_solver_finds_path() {
        let cells = vec![
            vec![Cell::Path, Cell::Wall, Cell::Path],
            vec![Cell::Path, Cell::Path, Cell::Path],
            vec![Cell::Wall, Cell::Path, Cell::Path],
        ];
        let start = (0, 0);
        let end = (2, 2);

        let maze = Maze { cells, start, end };

        let solver = BfsSolver;
        let path = solver.solve(&maze);

        assert!(path.is_some());
        let path = path.unwrap();

        // Ensure path starts at start and ends at end
        assert_eq!(path.first().cloned(), Some(start));
        assert_eq!(path.last().cloned(), Some(end));

        // Ensure all steps are adjacent and not through walls
        for i in 0..path.len() - 1 {
            let (r1, c1) = path[i];
            let (r2, c2) = path[i + 1];
            let dr = (r1 as isize - r2 as isize).abs();
            let dc = (c1 as isize - c2 as isize).abs();
            assert!(dr + dc == 1); // Manhattan distance must be 1
            assert_ne!(maze.cells[r2][c2], Cell::Wall);
        }
    }

    #[test]
    fn test_bfs_solver_no_path() {
        let cells = vec![
            vec![Cell::Path, Cell::Wall, Cell::Path],
            vec![Cell::Wall, Cell::Wall, Cell::Path],
            vec![Cell::Wall, Cell::Path, Cell::Path],
        ];
        let start = (0, 0);
        let end = (2, 2);

        let maze = Maze { cells, start, end };

        let solver = BfsSolver;
        let path = solver.solve(&maze);
        assert!(path.is_none()); // No valid path due to walls
    }
}
