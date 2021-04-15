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
