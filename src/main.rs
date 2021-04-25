mod algorithms;
mod benchmark_tests;
mod typing;

// use crate::algorithms::annealing_placement;
use crate::algorithms::genetic_placement;
use typing::{BLIFInfo, Problem};

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    pub n_generation: usize,
    pub n_population: usize,
    pub n_elite: usize,
    pub n_select: usize,
    pub n_crossover: usize,
    pub p_mutation: f32,
}

fn main() {
    let opts: Opts = Opts::parse();

    let filename = "benchmarks/alu2.blif";
    let info = BLIFInfo::from_file(filename);
    let problem = Problem::new(&info, 50, 40);
    let params = algorithms::GeneticParams {
        n_generation: opts.n_generation,
        n_population: opts.n_population,
        n_elite: opts.n_elite,
        n_select: opts.n_select,
        n_crossover: opts.n_crossover,
        p_mutation: opts.p_mutation,
    };
    println!("{:?}", params);
    let problem_json = serde_json::to_string(&problem).unwrap();
    println!("{}", problem_json);
    // // cost =  ; time =
    genetic_placement(&problem, &params);
}
