use crate::maze::Maze;

pub trait MazeGenerator {
    fn generate(
        &self,
        height: usize,
        width: usize,
        start: Option<(usize, usize)>,
        end: Option<(usize, usize)>,
        imperfect_percentage: f32,
    ) -> Maze;
}
