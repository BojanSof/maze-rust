use crate::maze::Maze;
use crate::progress::ProgressTracker;

pub trait Solver {
    fn solve(
        &self,
        maze: &Maze,
        tracker: Option<&mut ProgressTracker>,
    ) -> Option<Vec<(usize, usize)>>;
}
