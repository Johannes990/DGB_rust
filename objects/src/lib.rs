pub struct Cell {
    x: f64,
    y: f64,
    r: f64,
}

pub struct Rectangle {
    x_center: f64,
    y_center: f64,
    width: f64,
    height: f64,
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

impl Rectangle {
    pub fn new(start_x: f64, start_y: f64, end_x: f64, end_y: f64) -> Rectangle {
        let x_center: f64 = (start_x + end_x) / 2.0;
        let y_center: f64 = (start_y + end_y) / 2.0;
        let width: f64 = (end_x - start_x) / 2.0;
        let height: f64 = (end_y - start_y) / 2.0;

        Rectangle { x_center, y_center, width, height }
    }
}
