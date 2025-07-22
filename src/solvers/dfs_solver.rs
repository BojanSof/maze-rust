use std::collections::{HashMap, HashSet};

use crate::cell::Cell;
use crate::maze::Maze;
use crate::solvers::solver::Solver;
use crate::stack::Stack;

pub struct DfsSolver;

impl Solver for DfsSolver {
    fn solve(&self, maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let mut stack = Stack::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new(); // To track the path

        stack.push(maze.start);

        while let Some((row, col)) = stack.pop() {
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
                let neighbors =
                    Maze::get_neighbors(row, col, maze.cells.len(), maze.cells[0].len());
                for (n_row, n_col) in neighbors {
                    if maze.cells[n_row][n_col] != Cell::Wall && !visited.contains(&(n_row, n_col))
                    {
                        parent.insert((n_row, n_col), (row, col));
                        stack.push((n_row, n_col));
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
    use crate::solvers::solver::Solver; // adjust path as needed

    #[test]
    fn test_dfs_solver_finds_path() {
        let cells = vec![
            vec![
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
            ],
            vec![
                Cell::Wall,
                Cell::Path,
                Cell::Path,
                Cell::Path,
                Cell::Path,
                Cell::Path,
                Cell::Wall,
            ],
            vec![
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
            ],
        ];
        let start = (1, 1);
        let end = (1, 5);

        let maze = Maze { cells, start, end };

        let solver = DfsSolver;
        let path = solver.solve(&maze);

        assert!(path.is_some(), "Solver should find a path");
        let path = path.unwrap();

        assert_eq!(path.first().copied(), Some(start), "Path should start at S");
        assert_eq!(path.last().copied(), Some(end), "Path should end at E");

        for window in path.windows(2) {
            let (a, b) = (window[0], window[1]);

            let (a_row, a_col) = a;
            let (b_row, b_col) = b;

            assert!(
                maze.cells[b_row][b_col] != Cell::Wall,
                "Path goes through a wall at {:?}",
                b
            );

            let dr = (a_row as isize - b_row as isize).abs();
            let dc = (a_col as isize - b_col as isize).abs();
            assert!(
                dr + dc == 1,
                "Non-adjacent steps in path: {:?} -> {:?}",
                a,
                b
            );
        }
    }

    #[test]
    fn test_dfs_solver_no_path() {
        let cells = vec![
            vec![
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
            ],
            vec![
                Cell::Wall,
                Cell::Path,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Path,
                Cell::Wall,
            ],
            vec![
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
                Cell::Wall,
            ],
        ];
        let start = (1, 1);
        let end = (1, 5);

        let maze = Maze { cells, start, end };

        let solver = DfsSolver;
        let path = solver.solve(&maze);

        assert!(
            path.is_none(),
            "Solver should return None when no path exists"
        );
    }
}
