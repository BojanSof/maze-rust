use crate::cell::Cell;
use crate::generators::generator::MazeGenerator;
use crate::maze::Maze;
use crate::progress::ProgressTracker;
use rand::Rng;

pub struct AldousBroderMazeGenerator;

impl MazeGenerator for AldousBroderMazeGenerator {
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
        let mut unvisited = (height / 2) * (width / 2) - 1;

        let mut current_y = rng.gen_range(0..height / 2) * 2 + 1;
        let mut current_x = rng.gen_range(0..width / 2) * 2 + 1;
        cells[current_y][current_x] = Cell::Path;
        if let Some(ref mut t) = tracker {
            t.record(current_y, current_x, Cell::Path);
        }

        while unvisited > 0 {
            let mut neighbors = Vec::new();
            if current_y > 1 {
                neighbors.push((current_y - 2, current_x));
            }
            if current_y < height - 2 {
                neighbors.push((current_y + 2, current_x));
            }
            if current_x > 1 {
                neighbors.push((current_y, current_x - 2));
            }
            if current_x < width - 2 {
                neighbors.push((current_y, current_x + 2));
            }

            let (next_y, next_x) = neighbors[rng.gen_range(0..neighbors.len())];

            if cells[next_y][next_x] == Cell::Wall {
                cells[next_y][next_x] = Cell::Path;
                cells[(current_y + next_y) / 2][(current_x + next_x) / 2] = Cell::Path;
                if let Some(ref mut t) = tracker {
                    t.record(next_y, next_x, Cell::Path);
                    t.record(
                        (current_y + next_y) / 2,
                        (current_x + next_x) / 2,
                        Cell::Path,
                    );
                }
                unvisited -= 1;
            }
            current_y = next_y;
            current_x = next_x;
        }

        let start_node = start.unwrap_or((1, 1));
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
    fn test_generate_aldous_broder_maze() {
        let generator = AldousBroderMazeGenerator;
        let maze = generator.generate(11, 11, None, None, 0.0, None);
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.end, (9, 9));
        assert_eq!(maze.cells[maze.start.0][maze.start.1], Cell::Path);
        assert_eq!(maze.cells[maze.end.0][maze.end.1], Cell::Path);
    }
}
