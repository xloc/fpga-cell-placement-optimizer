use rand::seq::SliceRandom;

use crate::blif::BLIFInfo;
use crate::bound_box::BoundBox;
use crate::typing::{Coor, Net, Pin, PinID};

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
            self.coor2pin[ca.0][ca.1] = pb;
            self.coor2pin[cb.0][cb.1] = pa;
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

    pub fn cell_cost(&self, pins: &Vec<Pin>, nets: &Vec<Net>, coor: Coor) -> usize {
        let (x, y) = coor;
        if let Some(pin) = self.coor2pin[x][y] {
            let mut hp_cost = 0;
            for net_id in pins[pin].net_ids.iter() {
                let mut bb = BoundBox::new();
                for pin_id in &nets[*net_id].pins {
                    let coor = self.pin2coor[*pin_id];
                    bb.add_coor(coor);
                }
                hp_cost += bb.half_perimeter();
            }
            hp_cost
        } else {
            0
        }
    }
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[test]
fn it_should_swap_correctly() {
    let (nx, ny) = (4, 3);
    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![Some(0), None,    None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![None,    Some(1), None   ], // x=2
        vec![None,    None,    Some(2)], // x=3 
    ];

    let pin2coor = vec![(0, 0), (2, 1), (3, 2)];

    #[rustfmt::skip]
    let nets = vec![
        Net {id: 0, name: String::from("0"), pins: vec![0, 1]},
        Net {id: 1, name: String::from("1"), pins: vec![0, 2]},
    ];

    #[rustfmt::skip]
    let mut p = Placement { nx, ny, coor2pin, pin2coor };

    p.swap((0, 0), (2, 1));
    assert_eq!(p.coor2pin[0][0], Some(1));
    assert_eq!(p.coor2pin[2][1], Some(0));
    assert_eq!(p.pin2coor, vec![(2, 1), (0, 0), (3, 2)]);
    p.swap((0, 0), (2, 1));

    p.swap((0, 0), (1, 0));
    assert_eq!(p.coor2pin[0][0], None);
    assert_eq!(p.coor2pin[1][0], Some(0));
    assert_eq!(p.pin2coor, vec![(1, 0), (2, 1), (3, 2)]);
    p.swap((0, 0), (1, 0));

    p.swap((3, 1), (0, 0));
    assert_eq!(p.coor2pin[3][1], Some(0));
    assert_eq!(p.coor2pin[1][0], None);
    assert_eq!(p.pin2coor, vec![(3, 1), (2, 1), (3, 2)]);
}

#[test]
fn should_calculate_cost_correctly() {
    let (nx, ny) = (4, 3);
    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![Some(0), None,    None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![None,    Some(1), None   ], // x=2
        vec![None,    None,    Some(2)], // x=3 
    ];

    let pin2coor = vec![(0, 0), (2, 1), (3, 2)];

    #[rustfmt::skip]
    let nets = vec![
        Net {id: 0, name: String::from("0"), pins: vec![0, 1]},
        Net {id: 1, name: String::from("1"), pins: vec![0, 2]},
    ];

    #[rustfmt::skip]
    let mut p = Placement { nx, ny, coor2pin, pin2coor };

    assert_eq!(p.cost(&nets), 8);

    p.swap((0, 0), (2, 1));
    assert_eq!(p.cost(&nets), 5);
    p.swap((0, 0), (2, 1));

    p.swap((0, 0), (1, 0));
    assert_eq!(p.cost(&nets), 6);
    p.swap((0, 0), (1, 0));

    p.swap((3, 1), (0, 0));
    assert_eq!(p.cost(&nets), 2);
}
