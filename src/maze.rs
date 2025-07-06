use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::cell::Cell;

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
