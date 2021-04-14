use crate::typing::Coor;

pub struct BoundBox {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl BoundBox {
    pub fn new() -> Self {
        return Self {
            top: 0,
            bottom: 0,
            right: 0,
            left: 0,
        };
    }

    pub fn add_coor(&mut self, coor: Coor) {
        let (x, y) = coor;
        if x < self.left {
            self.left = x;
        } else if x > self.right {
            self.right = x;
        }
        if y < self.top {
            self.top = y;
        } else if y > self.bottom {
            self.bottom = y;
        }
    }

    pub fn half_perimeter(&self) -> usize {
        return (self.bottom - self.top) + (self.right - self.left);
    }
}
