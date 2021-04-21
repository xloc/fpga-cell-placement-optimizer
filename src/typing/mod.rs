mod blif;
mod bound_box;
mod placement;
mod problem;

use serde::{Deserialize, Serialize};

pub use blif::BLIFInfo;
pub use bound_box::BoundBox;
pub use placement::Placement;
pub use problem::make_coors;
pub use problem::Problem;

#[derive(Clone, Copy)]
pub struct Coor(pub usize, pub usize);

pub type PinID = usize;

#[derive(Serialize, Deserialize)]
pub struct Net {
    pub name: String,
    pub id: usize,
    pub pins: Vec<PinID>,
}

#[derive(Serialize, Deserialize)]
pub struct Pin {
    pub id: usize,
    pub net_ids: Vec<usize>,
}
