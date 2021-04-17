use rand::seq::SliceRandom;

use crate::placement::Placement;
use crate::problem::Problem;
// use crate::typing::Pin;
use crate::typing::{Coor, Net, Pin};

// fn mutate<'a>(mut assignment: Placement, coors: &Vec<Coor>, n_mutation: usize) -> Placement {
//     for _ in 0..n_mutation {
//         let ab = coors
//             .choose_multiple(&mut rand::thread_rng(), 2)
//             .collect::<Vec<_>>();
//         let (ca, cb) = (*ab[0], *ab[1]);

//         assignment.swap(ca, cb);
//     }
//     assignment
// }

pub struct Params {
    n_generation: usize,
    n_population: usize,
    reserve_ratio: f32,
    crossover_probability: f32,
    mutation_probability: f32,
    local_improvement_rate_probability: f32,
}

fn selection(population: &mut Vec<Placement>, n_reserve: usize) {
    // n_reserve placements are reserved during placement
    let selection_base = population.split_off(n_reserve);
    // do random selection on less good placements
    // weighted by costs
    // let selected = selection_base.
}

pub fn genetic_placement(problem: &Problem, params: &Params) {
    // init population
    let mut population: Vec<Placement> = Vec::new();
    for _ in 0..params.n_population {
        let sol = problem.make_placement();
        // println!("{}", sol.cost());
        population.push(sol);
    }

    let mut i_iter = 0;
    let mut is_converged = || -> bool {
        if i_iter > params.n_generation {
            true
        } else {
            i_iter += 1;
            false
        }
    };

    // loop {
    //     // compute fitness and sort
    //     population.sort_by_key(|i| i.cost());
    //     println!("{}", population[0].cost());
    //     // break if converge
    //     if is_converged() {
    //         break;
    //     }
    //     // selection
    //     // population.drain(n_parent..);
    //     population.drain(params.n_population..);
    //     let parents = &population[..n_parent];
    //     let mut offsprings: Vec<Placement> = Vec::new();

    //     // Revisiting Genetic Algorithms for the FPGA Placement Problem
    //     // crossover: Nope

    //     // mutation
    //     for parent in parents {
    //         for _ in 0..n_child_per_parent {
    //             let mutated = mutate((*parent).clone(), &coors, n_mutation_per_child);
    //             offsprings.push(mutated);
    //         }
    //     }
    //     population.extend(offsprings);

    //     // improvement
    // }
}
