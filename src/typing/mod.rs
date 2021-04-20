mod blif;
mod bound_box;
mod placement;
mod problem;

pub use blif::BLIFInfo;
pub use bound_box::BoundBox;
pub use placement::Placement;
pub use problem::make_coors;
pub use problem::Problem;

pub type Coor = (usize, usize);
pub type PinID = usize;

pub struct Net {
    pub name: String,
    pub id: usize,
    pub pins: Vec<PinID>,
}

pub struct Pin {
    pub id: usize,
    pub net_ids: Vec<usize>,
}
