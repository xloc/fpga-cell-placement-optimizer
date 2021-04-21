use rand::seq::SliceRandom;

use super::bound_box::BoundBox;
use super::problem::Problem;
use super::{Coor, PinID};

#[derive(Clone)]
pub struct Placement<'a> {
    pub problem: &'a Problem,

    pub pin2coor: Vec<Coor>,
    pub coor2pin: Vec<Vec<Option<PinID>>>,

    pub _cost: Option<usize>,
}

impl<'a> Placement<'a> {
    pub fn new(problem: &'a Problem) -> Self {
        let cell_assignment: Vec<Coor> = problem
            .coors
            .choose_multiple(&mut rand::thread_rng(), problem.n_pin)
            .map(|i| *i)
            .collect();

        let mut grid: Vec<Vec<Option<PinID>>> = vec![vec![None; problem.ny]; problem.nx];
        for (i_pin, coor) in cell_assignment.iter().enumerate() {
            let (x, y) = (coor.0, coor.1);
            grid[x][y] = Some(i_pin);
        }

        Self {
            problem: problem,
            coor2pin: grid,
            pin2coor: cell_assignment,
            _cost: None,
        }
    }

    pub fn swap(&mut self, ca: Coor, cb: Coor) {
        let before_cost = self.cell_cost(ca) + self.cell_cost(cb);

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

        let after_cost = self.cell_cost(ca) + self.cell_cost(cb);
        self._cost = Some(self.cost_mut() - before_cost + after_cost);
    }

    pub fn cost_mut(&mut self) -> usize {
        if let Some(cost) = self._cost {
            return cost;
        }
        self.cost_force()
    }

    pub fn cost_force(&mut self) -> usize {
        let mut hp_cost = 0;
        for net in self.problem.nets.iter() {
            let mut bb = BoundBox::new();
            for pin_id in &net.pins {
                let coor = self.pin2coor[*pin_id];
                bb.add_coor(coor);
            }
            hp_cost += bb.half_perimeter();
        }
        self._cost = Some(hp_cost);
        hp_cost
    }

    pub fn cost_panic(&self) -> usize {
        self._cost.unwrap()
    }

    pub fn cell_cost(&self, coor: Coor) -> usize {
        let (x, y) = (coor.0, coor.1);
        if let Some(pin) = self.coor2pin[x][y] {
            let mut hp_cost = 0;
            for net_id in self.problem.pins[pin].net_ids.iter() {
                let mut bb = BoundBox::new();
                for pin_id in &self.problem.nets[*net_id].pins {
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

    let pin2coor = vec![Coor(0, 0), Coor(2, 1), Coor(3, 2)];

    use crate::typing::Net;
    #[rustfmt::skip]
    let nets = vec![
        Net {id: 0, name: String::from("0"), pins: vec![0, 1]},
        Net {id: 1, name: String::from("1"), pins: vec![0, 2]},
    ];

    #[rustfmt::skip]
    let problem = Problem { nx, ny, nets, n_pin: 3, pins: vec![], coors: vec![] };
    #[rustfmt::skip]
    let mut p = Placement { problem: &problem, coor2pin, pin2coor, _cost:None };

    p.swap(Coor(0, 0), Coor(2, 1));
    assert_eq!(p.coor2pin[0][0], Some(1));
    assert_eq!(p.coor2pin[2][1], Some(0));
    assert_eq!(p.pin2coor, vec![(2, 1), (0, 0), (3, 2)]);
    p.swap(Coor(0, 0), Coor(2, 1));

    p.swap(Coor(0, 0), Coor(1, 0));
    assert_eq!(p.coor2pin[0][0], None);
    assert_eq!(p.coor2pin[1][0], Some(0));
    assert_eq!(p.pin2coor, vec![(1, 0), (2, 1), (3, 2)]);
    p.swap(Coor(0, 0), Coor(1, 0));

    p.swap(Coor(3, 1), Coor(0, 0));
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

    let pin2coor = vec![Coor(0, 0), Coor(2, 1), Coor(3, 2)];

    use crate::typing::Net;
    #[rustfmt::skip]
    let nets = vec![
        Net {id: 0, name: String::from("0"), pins: vec![0, 1]},
        Net {id: 1, name: String::from("1"), pins: vec![0, 2]},
    ];

    #[rustfmt::skip]
    let problem = Problem { nx, ny, nets, n_pin: 3, pins: vec![], coors: vec![] };
    #[rustfmt::skip]
    let mut p = Placement { problem: &problem, coor2pin, pin2coor, _cost: None };

    assert_eq!(p.cost_mut(), 8);

    p.swap(Coor(0, 0), Coor(2, 1));
    assert_eq!(p.cost_mut(), 5);
    p.swap(Coor(0, 0), Coor(2, 1));

    p.swap(Coor(0, 0), Coor(1, 0));
    assert_eq!(p.cost_mut(), 6);
    p.swap(Coor(0, 0), Coor(1, 0));

    p.swap(Coor(3, 1), Coor(0, 0));
    assert_eq!(p.cost_mut(), 2);
}
