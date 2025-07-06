use std::collections::{HashMap, HashSet};

use crate::cell::Cell;
use crate::maze::Maze;
use crate::solver::Solver;
use crate::stack::Stack;

pub struct DfsSolver;

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

impl Solver for DfsSolver {
    fn solve(&self, maze: &Maze) -> Option<Vec<(usize, usize)>> {
        let mut stack = Stack::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new(); // To track the path

        stack.push(maze.start);

        while let Some(&(row, col)) = stack.peek() {
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
            } else if maze.cells[row][col] == Cell::Path {
                let neighbors = get_neighbors(row, col, maze.cells.len(), maze.cells[0].len());
                if neighbors.is_empty() {
                    stack.pop();
                } else {
                    let mut found_unvisited = false;
                    for (n_row, n_col) in neighbors {
                        if maze.cells[n_row][n_col] != Cell::Wall
                            && !visited.contains(&(n_row, n_col))
                        {
                            parent.insert((n_row, n_col), (row, col));
                            stack.push((n_row, n_col));
                            found_unvisited = true;
                            break;
                        }
                    }
                    if !found_unvisited {
                        stack.pop(); // No unvisited neighbors, backtrack
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
    use crate::solver::Solver;

    use std::env;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_dfs_solver_finds_path() {
        let maze_text = "\
#######\n\
#S...E#\n\
#######";

        let mut path = env::temp_dir();
        path.push("test_dfs_maze.txt");

        {
            let mut file = File::create(&path).unwrap();
            write!(file, "{}", maze_text).unwrap();
        }

        let maze = Maze::from_file(&path).unwrap();
        let solver = DfsSolver;
        let result = solver.solve(&maze);

        assert!(result.is_some(), "Solver should find a path");
        let path = result.unwrap();

        assert_eq!(
            path.first().copied(),
            Some(maze.start),
            "Path should start at S"
        );
        assert_eq!(path.last().copied(), Some(maze.end), "Path should end at E");

        // Ensure all cells in path are not walls and adjacent
        for window in path.windows(2) {
            let (a, b) = (window[0], window[1]);

            let (a_row, a_col) = a;
            let (b_row, b_col) = b;

            assert!(
                maze.cells[b_row][b_col] != Cell::Wall,
                "Path goes through a wall"
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
}
