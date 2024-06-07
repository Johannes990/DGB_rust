pub struct Cell {
    x: f64,
    y: f64,
    r: f64,
}

impl Cell {
    pub fn new(x: f64, y: f64, r: f64) -> Cell {
        Cell { x, y, r }
    }

    pub fn move_cell(&mut self, dx: f64, dy: f64) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }

    pub fn resize_cell(&mut self, dr: f64) {
        self.r = self.r + dr;
    }
}
