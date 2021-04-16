mod blif;
use blif::BLIFInfo;
mod bound_box;
mod placement;
mod typing;

mod algorithms;
use crate::algorithms::annealing_placement;
use crate::algorithms::genetic_placement;

#[test]
fn pair_sa() {
    let filename = "benchmarks/pair.blif";
    let info = BLIFInfo::from_file(filename);
    // cost = 4507; time = 70.31s
    annealing_placement(&info, 50, 40, 5.0, 0.9, 0.1);
}

#[test]
fn apex1_sa() {
    let filename = "benchmarks/apex1.blif";
    let info = BLIFInfo::from_file(filename);
    // cost = 7325 ; time = 59.83s
    annealing_placement(&info, 50, 40, 5.0, 0.9, 0.1);
}

#[test]
fn alu2_sa() {
    let filename = "benchmarks/alu2.blif";
    let info = BLIFInfo::from_file(filename);
    // cost = 1394 ; time = 9.57s
    annealing_placement(&info, 50, 40, 5.0, 0.9, 0.1);
}

#[test]
fn genetic() {
    let filename = "benchmarks/pair.blif";
    let info = BLIFInfo::from_file(filename);
    // cost = 2010 ; time = 630s
    genetic_placement(&info, 50, 40, 100, 30, 1, 1);
}

fn main() {
    let filename = "benchmarks/apex1.blif"; // 35419 gene
    let filename = "benchmarks/alu2.blif"; // 4817 gene
    let filename = "benchmarks/pair.blif"; // 4817 gene
    let info = BLIFInfo::from_file(filename);
    annealing_placement(&info, 50, 40, 5., 0.9, 0.1);

    // genetic_placement(&info, 50, 40, 100, 30, 3, 10);
    // annealing_placement(&info, 35, 35, 5., 0.9, 0.01);
}
