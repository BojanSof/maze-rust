use crate::cell::Cell;

pub struct ProgressTracker {
    pub history: Vec<(usize, usize, Cell)>,
    pub enabled: bool,
}

impl ProgressTracker {
    pub fn new(enabled: bool) -> Self {
        Self {
            history: Vec::new(),
            enabled,
        }
    }

    pub fn record(&mut self, y: usize, x: usize, cell: Cell) {
        if self.enabled {
            self.history.push((y, x, cell));
        }
    }
}
