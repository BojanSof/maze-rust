use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::cell::Cell;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Maze {
    pub cells: Vec<Vec<Cell>>,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

#[derive(Debug)]
pub enum MazeError {
    Io(io::Error),
    InvalidCharacter(char, usize, usize),
    MissingStartOrEnd,
}

impl From<io::Error> for MazeError {
    fn from(err: io::Error) -> Self {
        MazeError::Io(err)
    }
}

impl Maze {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, MazeError> {
        let file = File::open(path)?;
        let reader = io::BufReader::new(file);

        let mut cells = Vec::new();
        let mut start = None;
        let mut end = None;

        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            let mut row = Vec::new();

            for (x, ch) in line.chars().enumerate() {
                let cell = match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Path,
                    'S' => {
                        if start.is_some() {
                            return Err(MazeError::InvalidCharacter(ch, y, x));
                        }
                        start = Some((y, x));
                        Cell::Path
                    }
                    'E' => {
                        if end.is_some() {
                            return Err(MazeError::InvalidCharacter(ch, y, x));
                        }
                        end = Some((y, x));
                        Cell::Path
                    }
                    _ => return Err(MazeError::InvalidCharacter(ch, y, x)),
                };
                row.push(cell);
            }

            cells.push(row);
        }

        let start = start.ok_or(MazeError::MissingStartOrEnd)?;
        let end = end.ok_or(MazeError::MissingStartOrEnd)?;

        Ok(Maze { cells, start, end })
    }

    pub fn get_neighbors(
        row: usize,
        col: usize,
        height: usize,
        width: usize,
    ) -> Vec<(usize, usize)> {
        const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut neighbors = Vec::new();

        for (dy, dx) in DIRS.iter() {
            let new_row = row as isize + dy;
            let new_col = col as isize + dx;

            if new_row >= 0 && new_row < height as isize && new_col >= 0 && new_col < width as isize
            {
                neighbors.push((new_row as usize, new_col as usize));
            }
        }

        neighbors
    }

    // converts maze to adjacency list representation, each edge is associated with a weight
    pub fn to_graph(&self) -> HashMap<(usize, usize), Vec<((usize, usize), usize)>> {
        let mut graph = HashMap::new();
        let rows = self.cells.len();
        let cols = self.cells[0].len();

        for i_row in 0..rows {
            for i_col in 0..cols {
                if self.cells[i_row][i_col] != Cell::Wall {
                    let neighbors = Maze::get_neighbors(i_row, i_col, rows, cols);
                    let mut adj_list = Vec::new();
                    for (n_row, n_col) in neighbors {
                        if self.cells[n_row][n_col] != Cell::Wall {
                            // Each edge has weight 1
                            adj_list.push(((n_row, n_col), 1));
                        }
                    }
                    graph.insert((i_row, i_col), adj_list);
                }
            }
        }

        graph
    }

    pub fn remove_walls(&mut self, percentage: f32) {
        if !(0.0..=100.0).contains(&percentage) {
            eprintln!("Percentage must be between 0.0 and 100.0");
            return;
        }

        let rows = self.cells.len();
        let cols = self.cells[0].len();
        let mut walls = Vec::new();

        for r in 1..rows - 1 {
            for c in 1..cols - 1 {
                if self.cells[r][c] == Cell::Wall {
                    walls.push((r, c));
                }
            }
        }

        let num_walls_to_remove = ((walls.len() as f32) * percentage / 100.0) as usize;
        let mut rng = rand::thread_rng();
        walls.shuffle(&mut rng);

        for i in 0..num_walls_to_remove {
            let (r, c) = walls[i];
            self.cells[r][c] = Cell::Path;
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i_row, row) in self.cells.iter().enumerate() {
            for (i_col, cell) in row.iter().enumerate() {
                let ch;
                if (i_row, i_col) == self.start {
                    ch = 'S';
                } else if (i_row, i_col) == self.end {
                    ch = 'E';
                } else {
                    ch = match cell {
                        Cell::Wall => '#',
                        Cell::Path => '.',
                    };
                }
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*; // brings Maze and MazeError into scope
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_maze_from_file_success() {
        // Prepare test maze string
        let maze_text = "\
#######\n\
#S...E#\n\
#######";

        // Create a temporary file
        let mut path = std::env::temp_dir();
        path.push("test_maze.txt");

        {
            let mut file = File::create(&path).unwrap();
            write!(file, "{}", maze_text).unwrap();
        }

        // Run the test
        let maze = Maze::from_file(&path).expect("Failed to parse maze");

        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.end, (1, 5));
    }

    #[test]
    fn test_maze_from_file_invalid_char() {
        let maze_text = "\
#####\n\
#S@E#\n\
#####";

        let mut path = std::env::temp_dir();
        path.push("test_maze_invalid.txt");

        {
            let mut file = File::create(&path).unwrap();
            write!(file, "{}", maze_text).unwrap();
        }

        let result = Maze::from_file(&path);
        assert!(matches!(
            result,
            Err(MazeError::InvalidCharacter('@', 1, 2))
        ));
    }

    #[test]
    fn test_maze_from_file_missing_start() {
        let maze_text = "\
#####\n\
#..E#\n\
#####";

        let mut path = std::env::temp_dir();
        path.push("test_maze_no_start.txt");

        {
            let mut file = File::create(&path).unwrap();
            write!(file, "{}", maze_text).unwrap();
        }

        let result = Maze::from_file(&path);
        assert!(matches!(result, Err(MazeError::MissingStartOrEnd)));
    }
}
