use rand::seq::SliceRandom;

use crate::blif::BLIFInfo;
use crate::bound_box::BoundBox;
use crate::typing::{Coor, Net, PinID};

#[derive(Clone)]
pub struct Placement {
    nx: usize,
    ny: usize,

    pin2coor: Vec<Coor>,
    coor2pin: Vec<Vec<Option<PinID>>>,
}

impl Placement {
    pub fn new(nx: usize, ny: usize, coors: &Vec<Coor>, blif: &BLIFInfo) -> Self {
        let cell_assignment: Vec<Coor> = coors
            .choose_multiple(&mut rand::thread_rng(), blif.n_pin)
            .map(|i| *i)
            .collect();

        let mut grid: Vec<Vec<Option<PinID>>> = vec![vec![None; ny]; nx];
        for (i_pin, coor) in cell_assignment.iter().enumerate() {
            let (x, y) = *coor;
            grid[x][y] = Some(i_pin);
        }

        Placement {
            nx,
            ny,
            coor2pin: grid,
            pin2coor: cell_assignment,
        }
    }

    pub fn swap(&mut self, ca: Coor, cb: Coor) {
        let pa = self.coor2pin[ca.0][ca.1];
        let pb = self.coor2pin[cb.0][cb.1];

        if pa == None && pb == None {
        } else if pa == None {
            self.coor2pin[ca.0][ca.1] = pb;
            self.coor2pin[cb.0][cb.1] = None;
            self.pin2coor[pb.unwrap()] = ca;
        } else if pb == None {
            self.coor2pin[cb.0][cb.1] = pa;
            self.coor2pin[ca.0][ca.1] = None;
            self.pin2coor[pa.unwrap()] = cb;
        } else {
            self.coor2pin[ca.0][ca.1] = pa;
            self.coor2pin[cb.0][cb.1] = pb;
            self.pin2coor[pb.unwrap()] = ca;
            self.pin2coor[pa.unwrap()] = cb;
        }
    }

    pub fn cost(&self, nets: &Vec<Net>) -> usize {
        let mut hp_cost = 0;
        for net in nets.iter() {
            let mut bb = BoundBox::new();
            for pin_id in &net.pins {
                let coor = self.pin2coor[*pin_id];
                bb.add_coor(coor);
            }
            hp_cost += bb.half_perimeter();
        }
        hp_cost
    }
}
