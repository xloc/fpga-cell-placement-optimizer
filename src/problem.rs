use crate::blif::BLIFInfo;
use crate::placement::Placement;
use crate::typing::{Coor, Net, Pin};

pub struct Problem {
    pub nx: usize,
    pub ny: usize,

    pub n_pin: usize,
    pub coors: Vec<Coor>,
    pub nets: Vec<Net>,
    pub pins: Vec<Pin>,
}

pub fn make_coors(nx: usize, ny: usize) -> Vec<Coor> {
    let mut coors: Vec<Coor> = Vec::new();
    for x in 0..nx {
        for y in 0..ny {
            coors.push((x, y));
        }
    }
    coors
}

impl Problem {
    pub fn new(blif: &BLIFInfo, nx: usize, ny: usize) -> Self {
        let coors = make_coors(nx, ny);

        let mut nets: Vec<Net> = Vec::new();
        let mut i_net = 0;
        for (name, pins) in blif.net_list.iter() {
            nets.push(Net {
                id: i_net,
                name: name.clone(),
                pins: pins.clone(),
            });
            i_net += 1;
        }

        let mut pins: Vec<Pin> = Vec::new();
        for i_pin in 0..blif.n_pin {
            pins.push(Pin {
                id: i_pin,
                net_ids: Vec::new(),
            });
        }
        for net in nets.iter() {
            for pin_id in &net.pins {
                pins[*pin_id].net_ids.push(net.id);
            }
        }

        Self {
            n_pin: blif.n_pin,
            nx,
            ny,
            coors,
            nets,
            pins,
        }
    }

    pub fn make_placement(&self) -> Placement {
        Placement::new(self)
    }
}
