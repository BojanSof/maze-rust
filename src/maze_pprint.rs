use crate::cell::Cell;
use crate::maze::Maze;
use std::collections::HashSet;

pub fn display_maze_with_path(maze: &Maze, path: &[(usize, usize)]) -> String {
    let mut result = String::new();
    let path_set: HashSet<(usize, usize)> = path.iter().copied().collect();

    for (row_idx, row) in maze.cells.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            let ch = if path_set.contains(&(row_idx, col_idx)) {
                // Prefer Start and End symbols even if in path
                if (row_idx, col_idx) == maze.start {
                    'S'
                } else if (row_idx, col_idx) == maze.end {
                    'E'
                } else {
                    '@'
                }
            } else {
                match cell {
                    Cell::Wall => '#',
                    Cell::Path => '.',
                }
            };
            result.push(ch);
        }
        result.push('\n');
    }

    result
}
