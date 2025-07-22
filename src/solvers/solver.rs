use crate::maze::Maze;

pub trait Solver {
    fn solve(&self, maze: &Maze) -> Option<Vec<(usize, usize)>>;
}
