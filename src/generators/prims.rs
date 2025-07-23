use crate::cell::Cell;
use crate::generators::generator::MazeGenerator;
use crate::maze::Maze;
use rand::Rng;

/// Generates a maze using Prim's algorithm.
pub struct PrimMazeGenerator;

impl MazeGenerator for PrimMazeGenerator {
    fn generate(
        &self,
        height: usize,
        width: usize,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
        imperfect_percentage: f32,
    ) -> Maze {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![Cell::Wall; width]; height];

        // List of walls. Each wall is (y, x, parent_y, parent_x)
        let mut walls = Vec::new();

        // Start at a random odd position
        let start_pos = (
            rng.gen_range(1..height / 2) * 2 + 1,
            rng.gen_range(1..width / 2) * 2 + 1,
        );
        cells[start_pos.0][start_pos.1] = Cell::Path;
        for (dy, dx) in [(-2i32, 0), (2, 0), (0, -2), (0, 2)] {
            let ny = start_pos.0 as i32 + dy;
            let nx = start_pos.1 as i32 + dx;
            if ny > 0 && ny < height as i32 && nx > 0 && nx < width as i32 {
                walls.push((ny as usize, nx as usize, start_pos.0, start_pos.1));
            }
        }

        while !walls.is_empty() {
            let idx = rng.gen_range(0..walls.len());
            let (wy, wx, py, px) = walls.swap_remove(idx);
            if cells[wy][wx] == Cell::Wall {
                // Find the cell on the opposite side
                let between_y = (wy + py) / 2;
                let between_x = (wx + px) / 2;
                if cells[between_y][between_x] == Cell::Wall {
                    cells[wy][wx] = Cell::Path;
                    cells[between_y][between_x] = Cell::Path;
                    // Add neighboring walls
                    for (dy, dx) in [(-2i32, 0), (2, 0), (0, -2), (0, 2)] {
                        let ny = wy as i32 + dy;
                        let nx = wx as i32 + dx;
                        if ny > 0 && ny < height as i32 && nx > 0 && nx < width as i32 {
                            if cells[ny as usize][nx as usize] == Cell::Wall {
                                walls.push((ny as usize, nx as usize, wy, wx));
                            }
                        }
                    }
                }
            }
        }

        // Set start and end
        let start = start.unwrap_or((1, 1));
        let end = end.unwrap_or((height - 2, width - 2));
        cells[start.0][start.1] = Cell::Path;
        cells[end.0][end.1] = Cell::Path;

        let mut maze = Maze { cells, start, end };

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
    fn test_generate_prim_maze() {
        // Generate a small maze
        let generator = PrimMazeGenerator;
        let maze = generator.generate(9, 9, None, None);
        // Ensure start and end are correct
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.end, (7, 7));
        // Ensure start and end are paths
        assert_eq!(maze.cells[maze.start.0][maze.start.1], Cell::Path);
        assert_eq!(maze.cells[maze.end.0][maze.end.1], Cell::Path);
        // Check there is at least one path from start to end using BFS
        let graph = maze.to_graph();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(maze.start);
        visited.insert(maze.start);
        let mut found = false;
        while let Some(pos) = queue.pop_front() {
            if pos == maze.end {
                found = true;
                break;
            }
            if let Some(neighbors) = graph.get(&pos) {
                for (neighbor, _) in neighbors {
                    if visited.insert(*neighbor) {
                        queue.push_back(*neighbor);
                    }
                }
            }
        }
        assert!(found, "No path from start to end in Prim maze");
    }

    #[test]
    fn test_generate_prim_maze_with_custom_start_end() {
        // Generate a small maze with custom start and end
        let start = (3, 3);
        let end = (5, 5);
        let generator = PrimMazeGenerator;
        let maze = generator.generate(9, 9, Some(start), Some(end));
        // Ensure start and end are correct
        assert_eq!(maze.start, start);
        assert_eq!(maze.end, end);
        // Ensure start and end are paths
        assert_eq!(maze.cells[maze.start.0][maze.start.1], Cell::Path);
        assert_eq!(maze.cells[maze.end.0][maze.end.1], Cell::Path);
    }
}
