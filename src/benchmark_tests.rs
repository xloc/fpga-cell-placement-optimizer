#[allow(unused_imports)]
use crate::algorithms;
#[allow(unused_imports)]
use crate::algorithms::annealing_placement;
#[allow(unused_imports)]
use crate::algorithms::genetic_placement;
#[allow(unused_imports)]
use crate::typing::{BLIFInfo, Problem};

#[test]
fn pair_sa() {
    let filename = "benchmarks/pair.blif";
    let info = BLIFInfo::from_file(filename);
    let problem = Problem::new(&info, 50, 40);
    let params = algorithms::AnnealingParams {
        t_init: 5.0,
        t_decrease_factor: 0.9,
        t_terminate: 0.1,
    };
    // cost = 4507; time = 70.31s
    annealing_placement(&problem, &params);
}

#[test]
fn apex1_sa() {
    let filename = "benchmarks/apex1.blif";
    let info = BLIFInfo::from_file(filename);
    let problem = Problem::new(&info, 50, 40);
    let params = algorithms::AnnealingParams {
        t_init: 5.0,
        t_decrease_factor: 0.9,
        t_terminate: 0.1,
    };
    // cost = 7325 ; time = 59.83s
    annealing_placement(&problem, &params);
}

#[test]
fn alu2_sa() {
    let filename = "benchmarks/alu2.blif";
    let info = BLIFInfo::from_file(filename);
    let problem = Problem::new(&info, 50, 40);
    let params = algorithms::AnnealingParams {
        t_init: 5.0,
        t_decrease_factor: 0.9,
        t_terminate: 0.1,
    };
    // cost = 1394 ; time = 9.57s
    annealing_placement(&problem, &params);
}

#[test]
fn genetic() {
    let filename = "benchmarks/alu2.blif";
    let info = BLIFInfo::from_file(filename);
    let problem = Problem::new(&info, 50, 40);
    let params = algorithms::GeneticParams {
        n_generation: 1000_000,
        n_population: 100,
        n_elite: 10,
        n_select: 80,
        n_crossover: 10,
        p_mutation: 0.5,
    };
    println!(
        "\"cross={:.2}, mut={:.2}\"",
        params.n_crossover, params.p_mutation
    );
    // cost =  ; time =
    genetic_placement(&problem, &params);
}
