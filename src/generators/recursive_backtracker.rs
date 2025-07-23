use crate::cell::Cell;
use crate::generators::generator::MazeGenerator;
use crate::maze::Maze;
use crate::progress::ProgressTracker;
use rand::seq::IndexedRandom;

pub struct RecursiveBacktrackerMazeGenerator;

impl MazeGenerator for RecursiveBacktrackerMazeGenerator {
    fn generate(
        &self,
        height: usize,
        width: usize,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
        imperfect_percentage: f32,
        mut tracker: Option<&mut ProgressTracker>,
    ) -> Maze {
        let mut cells = vec![vec![Cell::Wall; width]; height];
        let mut rng = rand::thread_rng();
        let mut stack = Vec::new();

        let start_node = start.unwrap_or((1, 1));
        cells[start_node.0][start_node.1] = Cell::Path;
        if let Some(ref mut t) = tracker {
            t.record(start_node.0, start_node.1, Cell::Path);
        }
        stack.push(start_node);

        while let Some(&(y, x)) = stack.last() {
            let mut neighbors = Vec::new();
            // Check neighbors (2 cells away)
            // North
            if y > 1 && cells[y - 2][x] == Cell::Wall {
                neighbors.push((y - 2, x));
            }
            // South
            if y < height - 2 && cells[y + 2][x] == Cell::Wall {
                neighbors.push((y + 2, x));
            }
            // West
            if x > 1 && cells[y][x - 2] == Cell::Wall {
                neighbors.push((y, x - 2));
            }
            // East
            if x < width - 2 && cells[y][x + 2] == Cell::Wall {
                neighbors.push((y, x + 2));
            }

            if let Some(&next_cell) = neighbors.choose(&mut rng) {
                let (next_y, next_x) = next_cell;
                // Carve path to neighbor
                cells[next_y][next_x] = Cell::Path;
                cells[(y + next_y) / 2][(x + next_x) / 2] = Cell::Path;
                if let Some(ref mut t) = tracker {
                    t.record(next_y, next_x, Cell::Path);
                    t.record((y + next_y) / 2, (x + next_x) / 2, Cell::Path);
                }
                stack.push(next_cell);
            } else {
                stack.pop();
            }
        }

        let end_node = end.unwrap_or((height - 2, width - 2));
        cells[start_node.0][start_node.1] = Cell::Path;
        cells[end_node.0][end_node.1] = Cell::Path;

        let mut maze = Maze {
            cells,
            start: start_node,
            end: end_node,
        };

        if imperfect_percentage > 0.0 {
            maze.remove_walls(imperfect_percentage);
        }

        maze
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell;
    use crate::generators::generator::MazeGenerator;

    #[test]
    fn test_generate_recursive_backtracker_maze() {
        let generator = RecursiveBacktrackerMazeGenerator;
        let maze = generator.generate(11, 11, None, None, 0.0, None);
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.end, (9, 9));
        assert_eq!(maze.cells[maze.start.0][maze.start.1], Cell::Path);
        assert_eq!(maze.cells[maze.end.0][maze.end.1], Cell::Path);
    }
}
