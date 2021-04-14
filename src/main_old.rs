// mod blif;
// use blif::{BLIFInfo, PinID};
// use rand::seq::SliceRandom;
// use std::collections::HashSet;
// use std::hash::{Hash, Hasher};

// fn main() {
//     let filename = "./apex1.blif";
//     let blif = BLIFInfo::from_file(filename);

//     println!("{}", blif.n_pin);
//     println!("{:?}", blif.net_list.len());

//     genetic_placement(&blif, 50, 22, 38);
// }

// type NetID = usize;

// struct Net {
//     net_name: String,
//     net_id: usize,
//     pin_ids: Vec<PinID>,
// }
// impl Hash for Net {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.net_name.hash(state);
//     }
// }

// struct Chip {
//     nx: usize,
//     ny: usize,

//     cells: Vec<Coor2D>,
//     nets: Vec<Net>,
// }

// // struct GeneticConfig {
// //     parent_ratio: f64,
// //     n_population: usize,
// // }

// type Coor2D = (usize, usize);

// // impl Chip {
// //     fn new(blif: BLIFInfo, nx: usize, ny: usize) -> Self {
// //         let mut coors = Vec::<Coor2D>::new();
// //         for x in 1..nx {
// //             for y in 1..ny {
// //                 coors.push((x, y));
// //             }
// //         }

// //         let cell_assignment: Vec<_> = coors
// //             .choose_multiple(&mut rand::thread_rng(), blif.net_list.len())
// //             .collect();

// //         let grid: Vec<Vec<Option<Vec<&Net>>>> = vec![vec![None; ny]; nx];
// //         let nets = Vec::new();
// //         let mut i_net = 0;
// //         for (net_name, pin_ids) in blif.net_list {
// //             let net = Net {
// //                 net_id: i_net,
// //                 net_name,
// //                 pin_ids,
// //             };
// //             nets.push(&net);
// //             i_net += 1;
// //             for pin_id in pin_ids {
// //                 let (px, py) = cell_assignment[pin_id];
// //             }
// //         }

// //         Chip { nx, ny, cells }
// //     }
// // }

// type CellShape = (usize, usize);

// fn make_coors(nx: usize, ny: usize) -> Vec<Coor2D> {
//     let mut coors = Vec::<Coor2D>::new();
//     for x in 0..nx {
//         for y in 0..ny {
//             coors.push((x, y));
//         }
//     }
//     coors
// }

// struct Cell<'a> {
//     coor: Coor2D,
//     nets: Option<Vec<&'a Net>>,
// }

// fn genetic_placement(blif: &BLIFInfo, n_population: usize, nx: usize, ny: usize) {
//     // let chip = Vec::<Chip>::new();
//     let coors = make_coors(nx, ny);

//     let nets = blif.net_list.iter().map(|it| {it.})
//     for i in 0..n_population {
//         // what should be inside a chip?
//         // random assign pins to cells
//         // The pins and net are the same
//         // pins are Vec of Nets
//         // pins are assigned to cells of chips
//         // to calculate half-perimeter of all nets we need to keep track of all pin2cell assignments.
//         let cell_assignment: Vec<&(usize, usize)> = coors
//             .choose_multiple(&mut rand::thread_rng(), blif.net_list.len())
//             .collect();

//         let grid: Vec<Vec<Option<Vec<&Net>>>> = vec![vec![None; ny]; nx];
//         let mut nets = Vec::new();
//         let mut i_net = 0;
//         for (net_name, pin_ids) in blif.net_list.iter() {
//             let net = Net {
//                 net_id: i_net,
//                 net_name: net_name.clone(),
//                 pin_ids: pin_ids.clone(),
//             };
//             nets.push(net);
//             i_net += 1;

//             for pin_id in pin_ids.iter() {
//                 let (x, y) = cell_assignment[*pin_id];

//                 match grid[*x][*y] {
//                     None =>
//                 }
//                 grid[*x][*y].unwrap_or()
//             }
//         }
//     }

//     // init population
//     loop {
//         // compute fitness
//         let converged = true;
//         if converged {
//             break;
//         }
//         // selection
//         // crossover
//         // mutation
//     }
// }

// // fn annealing_placement(t_init: f32, n_batch: usize) {
// //     let t = t_init;
// //     loop {
// //         for _ in 0..n_batch {
// //             // randomly select two pins

// //             // calculate previous cost
// //             // swap pin position
// //             // calculate current cost
// //             // calculate delta cost
// //             let delta_cost = 10 as f32; // dummy

// //             let r = 0.1; // some random number; dummy
// //             if r < f32::exp(-delta_cost / t) { // confirm swap
// //             } else { // restore swap
// //             }
// //         }
// //         // decrease t
// //         if should_terminate() {
// //             break;
// //         }
// //     }
// // }
