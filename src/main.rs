mod blif;
use blif::BLIFInfo;
mod bound_box;
mod placement;
mod problem;
mod typing;

mod algorithms;
use crate::algorithms::annealing_placement;
use crate::algorithms::genetic_placement;
use problem::Problem;

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
        n_generation: 26_000,
        n_population: 100,
        reserve_ratio: 0.2,
        random_reserve_ratio: 0.2,
        crossover_probability: 1.0,
        mutation_probability: 1.0,
        local_improvement_rate_probability: 0.05,
    };
    println!(
        "\"cross={:.2}, mut={:.2}\"",
        params.crossover_probability, params.mutation_probability
    );
    println!(":[");
    // cost =  ; time =
    genetic_placement(&problem, &params);
    println!("],");
}

fn main() {
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

    let params = algorithms::GeneticParams {
        n_generation: 1_000_000,
        n_population: 100,
        reserve_ratio: 0.2,
        random_reserve_ratio: 0.3,
        crossover_probability: 0.3,
        mutation_probability: 0.05,
        local_improvement_rate_probability: 0.05,
    };
    // cost =  ; time =
    genetic_placement(&problem, &params);

    // genetic_placement(&info, 50, 40, 100, 30, 3, 10);
    // annealing_placement(&info, 35, 35, 5., 0.9, 0.01);
}
