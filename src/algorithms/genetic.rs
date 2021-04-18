use rand::seq::SliceRandom;
use rand::Rng;

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
    pub n_generation: usize,
    pub n_population: usize,
    pub reserve_ratio: f32,
    pub random_reserve_ratio: f32,
    pub crossover_probability: f32,
    pub mutation_probability: f32,
    pub local_improvement_rate_probability: f32,
}

fn selection(mut population: Vec<Placement>, n_fixed: usize, n_random: usize) -> Vec<Placement> {
    // n_reserve placements are reserved during placement
    let mut selection_base = population.split_off(n_fixed);
    // do random selection on less good placements
    // weighted by costs
    let cost_max = selection_base.last_mut().unwrap().cost_mut();
    let mut fitnesses: Vec<(usize, usize)> = selection_base
        .iter_mut()
        .map(|i| cost_max - i.cost_mut() + (cost_max as f32 * 0.01) as usize)
        .enumerate()
        .collect();
    fitnesses.shuffle(&mut rand::thread_rng());
    // println!("{:?}", fitnesses);
    let fitness_sum = fitnesses.iter().fold(0, |acc, (i, fit)| acc + fit);
    let arc_len = fitness_sum / n_random;
    let random_offset = (rand::thread_rng().gen::<f32>() * arc_len as f32) as usize;
    let mut selected_index = Vec::new();
    let mut pos = random_offset;
    let mut acc = 0;
    for (i, fit) in fitnesses {
        acc += fit;
        if acc > pos {
            selected_index.push(i);
            pos += arc_len;
        }
    }
    // println!("{:?}", selected_index);

    let mut opt_selection_base = Vec::new();
    for p in selection_base {
        opt_selection_base.push(Some(p));
    }

    for i in selected_index {
        let p = std::mem::replace(&mut opt_selection_base[i], None);
        if let Some(p) = p {
            population.push(p);
        } else {
            println!("select twice");
        }
    }
    population
}

fn make_fixture() -> Problem {
    let filename = "benchmarks/alu2.blif";
    use crate::blif::BLIFInfo;
    let info = BLIFInfo::from_file(filename);
    Problem::new(&info, 50, 40)
}

#[test]
fn test_selection() {
    let problem = make_fixture();
    let mut population: Vec<Placement> = (0..10).map(|i| problem.make_placement()).collect();
    population.iter_mut().for_each(|p| {
        p.cost_mut();
    });
    population.sort_by_cached_key(|i| i.cost_panic());
    println!(
        "{:?}",
        population
            .iter_mut()
            .map(|i| i.cost_mut())
            .collect::<Vec<usize>>()
    );
    let mut selected = selection(population, 3, 3);
    println!(
        "{:?}",
        selected
            .iter_mut()
            .map(|i| i.cost_mut())
            .collect::<Vec<usize>>()
    );
}

fn mutate(mut placement: Placement) -> Placement {
    let (ca, cb) = super::util::take_2(&placement.problem.coors);
    placement.swap(ca, cb);
    placement
}

fn improve(mut placement: Placement) -> Option<Placement> {
    let (ca, cb) = super::util::take_2(&placement.problem.coors);
    let prev = placement.cell_cost(ca) + placement.cell_cost(cb);
    placement.swap(ca, cb);
    let curr = placement.cell_cost(ca) + placement.cell_cost(cb);
    if prev < curr {
        Some(placement)
    } else {
        None
    }
}

fn crossover<'a>(a: &'a Placement, b: &'a Placement) -> (Placement<'a>, Placement<'a>) {
    let a = a.clone();
    let b = b.clone();

    (a, b)
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

    let n_reserve = (params.reserve_ratio * params.n_population as f32) as usize;
    let n_random_reserve = (params.random_reserve_ratio * params.n_population as f32) as usize;
    let rng = &mut rand::thread_rng();
    loop {
        // compute fitness and sort
        population.iter_mut().for_each(|p| {
            p.cost_mut();
        });
        population.sort_by_cached_key(|i| i.cost_panic());
        if i_iter % 1000 == 0 {
            println!("{}", population[0].cost_mut());
        }
        // break if converge
        if i_iter > params.n_generation {
            break;
        } else {
            i_iter += 1;
        }
        // selection
        let mut survived = selection(population, n_reserve, n_random_reserve);

        // FPGA PLACEMENT OPTIMIZATION BY TWO-STEP UNIFIED GENETIC ALGORITHM AND SIMULATED ANNEALING ALGORITHM
        // crossover
        // let mut crossed = Vec::new();
        // let parents = &survived;
        // let mut parent_index = (0..parents.len() * 2 / 2).collect::<Vec<usize>>();
        // parent_index.shuffle(rng);
        // let mut parents_index_iter = parent_index.iter();
        // for _ in 0..parents.len() / 2 {
        //     let ia = *parents_index_iter.next().unwrap();
        //     let ib = *parents_index_iter.next().unwrap();
        //     let (c, d) = crossover(&parents[ia], &parents[ib]);
        //     crossed.push(c);
        //     crossed.push(d);
        // }

        // mutation
        let mut mutated = Vec::new();
        // TODO: don't mutate all
        for p in survived.iter() {
            let mp = mutate(p.clone());
            mutated.push(mp);
        }

        // improvement
        let improved = improve(survived.choose(rng).unwrap().clone());

        // improvement and join population
        population = survived;
        population.extend(mutated);
        // population.extend(crossed);
        if let Some(p) = improved {
            population.push(p);
        }
    }
}
