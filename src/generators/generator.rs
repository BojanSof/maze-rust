use crate::maze::Maze;
use crate::progress::ProgressTracker;

pub trait MazeGenerator {
    fn generate(
        &self,
        height: usize,
        width: usize,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
        imperfect_percentage: f32,
        tracker: Option<&mut ProgressTracker>,
    ) -> Maze;
}
