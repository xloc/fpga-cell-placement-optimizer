use crate::typing::Coor;

#[derive(Debug)]
pub struct BoundBox {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
    initialized: bool,
}

impl BoundBox {
    pub fn new() -> Self {
        return Self {
            top: 0,
            left: 0,
            bottom: 0,
            right: 0,
            initialized: false,
        };
    }

    pub fn add_coor(&mut self, coor: Coor) {
        let (x, y) = coor;

        if !self.initialized {
            self.top = y;
            self.bottom = y;
            self.left = x;
            self.right = x;
            self.initialized = true;
            return;
        }
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
