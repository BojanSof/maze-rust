use crate::cell::Cell;
use crate::generators::generator::MazeGenerator;
use crate::maze::Maze;
use rand::seq::SliceRandom;

pub struct KruskalMazeGenerator;

struct DisjointSet {
    parent: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        self.parent[i] = self.find(self.parent[i]);
        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
        }
    }
}

impl MazeGenerator for KruskalMazeGenerator {
    fn generate(
        &self,
        height: usize,
        width: usize,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
    ) -> Maze {
        let mut cells = vec![vec![Cell::Wall; width]; height];
        let mut rng = rand::thread_rng();
        let mut walls = Vec::new();

        for r in (1..height).step_by(2) {
            for c in (1..width).step_by(2) {
                cells[r][c] = Cell::Path;
                if r + 2 < height {
                    walls.push((r, c, r + 2, c));
                }
                if c + 2 < width {
                    walls.push((r, c, r, c + 2));
                }
            }
        }

        let mut dset = DisjointSet::new(height * width);
        walls.shuffle(&mut rng);

        for (r1, c1, r2, c2) in walls {
            let idx1 = r1 * width + c1;
            let idx2 = r2 * width + c2;
            if dset.find(idx1) != dset.find(idx2) {
                dset.union(idx1, idx2);
                cells[(r1 + r2) / 2][(c1 + c2) / 2] = Cell::Path;
            }
        }

        let start_node = start.unwrap_or((1, 1));
        let end_node = end.unwrap_or((height - 2, width - 2));
        cells[start_node.0][start_node.1] = Cell::Path;
        cells[end_node.0][end_node.1] = Cell::Path;

        Maze {
            cells,
            start: start_node,
            end: end_node,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::Cell;
    use crate::generators::generator::MazeGenerator;

    #[test]
    fn test_generate_kruskal_maze() {
        let generator = KruskalMazeGenerator;
        let maze = generator.generate(11, 11, None, None);
        assert_eq!(maze.start, (1, 1));
        assert_eq!(maze.end, (9, 9));
        assert_eq!(maze.cells[maze.start.0][maze.start.1], Cell::Path);
        assert_eq!(maze.cells[maze.end.0][maze.end.1], Cell::Path);
    }
}
