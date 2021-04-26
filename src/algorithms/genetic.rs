use rand::seq::SliceRandom;
use rand::Rng;

use crate::typing::Placement;
use crate::typing::Problem;
use crate::typing::{Coor, PinID};

#[derive(Debug)]
pub struct Params {
    pub n_generation: usize,
    pub n_population: usize,
    pub n_elite: usize,
    pub n_select: usize,
    pub n_crossover: usize,
    pub p_mutation: f32,
}

fn selection(
    mut selection_base: Vec<Placement>,
    n_survive: usize,
    padding_cost: bool,
) -> Vec<Placement> {
    let cost_max = selection_base.last_mut().unwrap().cost_mut();
    let mut fitnesses: Vec<(usize, usize)> = if padding_cost {
        let padding = (cost_max as f32 * 0.01) as usize;
        selection_base
            .iter_mut()
            .map(|i| cost_max - i.cost_mut() + padding)
            .enumerate()
            .collect()
    } else {
        selection_base
            .iter_mut()
            .map(|i| cost_max - i.cost_mut())
            .enumerate()
            .collect()
    };

    let rng = &mut rand::thread_rng();
    fitnesses.shuffle(rng);
    let fitness_sum = fitnesses.iter().fold(0, |acc, (_, fit)| acc + fit);
    let arc_len = fitness_sum / n_survive;
    let random_offset = (rng.gen::<f32>() * arc_len as f32) as usize;

    let mut is_selected = vec![false; selection_base.len()];
    let mut pos = random_offset;
    let mut acc = 0;
    for (i, fitness) in fitnesses {
        acc += fitness;
        if acc > pos {
            is_selected[i] = true;
            pos += arc_len;
        }
    }
    // println!("{:?}", selected_index);

    selection_base
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| if is_selected[i] { Some(p) } else { None })
        .collect::<Vec<Placement>>()
}

#[allow(dead_code)]
fn make_fixture() -> Problem {
    let filename = "benchmarks/alu2.blif";
    use crate::typing::BLIFInfo;
    let info = BLIFInfo::from_file(filename);
    Problem::new(&info, 50, 40)
}

#[test]
fn test_selection() {
    let problem = make_fixture();
    let mut population: Vec<Placement> = (0..10).map(|_| problem.make_placement()).collect();
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
    let mut selected = selection(population, 6, true);
    println!(
        "{:?}",
        selected
            .iter_mut()
            .map(|i| i.cost_mut())
            .collect::<Vec<usize>>()
    );
}

fn mutate(placement: &mut Placement, i_iter: &usize) {
    let n_swap = if i_iter < &10_000 { 10 } else { 2 };
    // for _ in 0..rand::thread_rng().gen_range(1..2) {
    for _ in 0..1 {
        let (ca, cb) = super::util::take_2(&placement.problem.coors);
        placement.swap(ca, cb);
    }
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

fn crossover<'a>(a: &Placement, b: &Placement, c: &'a mut Placement, d: &'a mut Placement) {
    let problem = a.problem;
    let i_divide = (problem.nx as f32 * rand::thread_rng().gen::<f32>()) as usize;

    crossover_half(a, b, d, i_divide, &mut rand::thread_rng());
    crossover_half(b, a, c, i_divide, &mut rand::thread_rng());
}

fn crossover_half<'a, R: Rng>(
    a: &Placement,
    b: &Placement,
    out: &'a mut Placement,
    i_divide: usize,
    rng: &mut R,
) {
    // TODO: vertical division
    let problem = a.problem;

    let mut empty_coors = Vec::new();

    // copy b.right to out.right
    let mut out_p2c: Vec<Option<Coor>> = vec![None; problem.n_pin];
    for x in i_divide..problem.nx {
        for y in 0..problem.ny {
            if let Some(pin) = b.coor2pin[x][y] {
                out_p2c[pin] = Some(Coor(x, y));
            } else {
                empty_coors.push(Coor(x, y));
            }
        }
    }

    // copy a.left to out.left if not duplicated
    for x in 0..i_divide {
        for y in 0..problem.ny {
            if let Some(pin) = a.coor2pin[x][y] {
                // ^ if coor (x, y) contains a pin
                if let Some(_) = out_p2c[pin] {
                    // ^ if duplicates: pin[a.left] already in c.right
                    // println!("duplicate :pin={}, coor={:?}", pin, dup_coor);
                    // duplicated_pins
                    empty_coors.push(Coor(x, y));
                } else {
                    // ^ no duplication
                    out.coor2pin[x][y] = a.coor2pin[x][y];
                    out_p2c[pin] = Some(Coor(x, y));
                }
            } else {
                out.coor2pin[x][y] = None;
                empty_coors.push(Coor(x, y));
            }
        }
    }

    for (pin_id, _) in out_p2c.iter().enumerate().filter(|(_, p)| p.is_none()) {
        let select_i = rng.gen_range(0..empty_coors.len());
        let coor = empty_coors.swap_remove(select_i);
        out.coor2pin[coor.0][coor.1] = Some(pin_id);
    }

    let mut d_new_pin2coor: Vec<Option<Coor>> = vec![None; problem.n_pin];
    for x in 0..problem.nx {
        for y in 0..problem.ny {
            if let Some(pin) = out.coor2pin[x][y] {
                d_new_pin2coor[pin] = Some(Coor(x, y));
            }
        }
    }

    for i_pin in 0..problem.n_pin {
        out.pin2coor[i_pin] = d_new_pin2coor[i_pin].unwrap();
    }
}

#[test]
fn should_crossover_without_problem() {
    let problem = make_fixture();
    let a = problem.make_placement();
    let b = problem.make_placement();
    let mut out = b.clone();
    crossover_half(&a, &b, &mut out, problem.nx / 2, &mut rand::thread_rng());
}

#[allow(dead_code)]
fn derive_pin2coor(
    coor2pin: &Vec<Vec<Option<PinID>>>,
    nx: usize,
    ny: usize,
    n_pin: usize,
) -> Vec<Coor> {
    let mut d_new_pin2coor: Vec<Option<Coor>> = vec![None; n_pin];
    for x in 0..nx {
        for y in 0..ny {
            if let Some(pin) = coor2pin[x][y] {
                d_new_pin2coor[pin] = Some(Coor(x, y));
            }
        }
    }
    let mut pin2coor = Vec::new();
    for i_pin in 0..n_pin {
        pin2coor.push(d_new_pin2coor[i_pin].unwrap());
    }
    pin2coor
}

#[test]
fn should_crossover_if_no_overlap() {
    let (nx, ny) = (4, 3);
    let n_pin = 3;

    use crate::typing::make_coors;
    #[rustfmt::skip]
    let problem = Problem { nx, ny, nets: vec![], n_pin, pins: vec![], coors: make_coors(nx, ny) };

    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![None   , Some(0), None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![Some(1), None  ,  None ], // x=2
        vec![None,    Some(2), None], // x=3 
    ];
    let pin2coor = derive_pin2coor(&coor2pin, nx, ny, n_pin);
    #[rustfmt::skip]
    let a = Placement { problem: &problem, coor2pin, pin2coor, _cost:None };

    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![Some(0), None,    None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![None,    Some(1), None   ], // x=2
        vec![None,    None,    Some(2)], // x=3 
    ];
    let pin2coor = derive_pin2coor(&coor2pin, nx, ny, n_pin);
    #[rustfmt::skip]
    let b = Placement { problem: &problem, coor2pin, pin2coor, _cost:None };

    let mut out = b.clone();
    crossover_half(&a, &b, &mut out, 2, &mut rand::thread_rng());

    print_coor2pin(&problem, &out.coor2pin);
    #[rustfmt::skip]
    assert_eq!(&out.coor2pin, &vec![
        //   y=0      y=1      y=2
        vec![None,    Some(0), None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![None,    Some(1), None   ], // x=2
        vec![None,    None,    Some(2)], // x=3 
    ]);
}

#[test]
fn should_crossover_if_a_left_b_right_do_not_cover_all_pins() {
    let (nx, ny) = (4, 3);
    let n_pin = 3;

    use crate::typing::make_coors;
    #[rustfmt::skip]
    let problem = Problem { nx, ny, nets: vec![], n_pin, pins: vec![], coors: make_coors(nx, ny) };

    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![None   , Some(0), None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![Some(1), None  ,  None   ], // x=2
        vec![None,    Some(2), None   ], // x=3 
    ];
    let pin2coor = derive_pin2coor(&coor2pin, nx, ny, n_pin);
    #[rustfmt::skip]
    let a = Placement { problem: &problem, coor2pin, pin2coor, _cost:None };

    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![Some(0), Some(1), None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![None,    None,    None   ], // x=2
        vec![None,    None,    Some(2)], // x=3 
    ];
    let pin2coor = derive_pin2coor(&coor2pin, nx, ny, n_pin);
    #[rustfmt::skip]
    let b = Placement { problem: &problem, coor2pin, pin2coor, _cost:None };

    let mut out = b.clone();
    crossover_half(&a, &b, &mut out, 2, &mut rand::thread_rng());

    print_coor2pin(&problem, &out.coor2pin);
    assert_eq!(&out.coor2pin[3][2], &Some(2));
    assert_eq!(&out.coor2pin[0][1], &Some(0));
}

#[test]
fn should_crossover_if_a_left_b_right_have_overlapped_pins() {
    let (nx, ny) = (4, 3);
    let n_pin = 3;

    use crate::typing::make_coors;
    #[rustfmt::skip]
    let problem = Problem { nx, ny, nets: vec![], n_pin, pins: vec![], coors: make_coors(nx, ny) };

    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![Some(1), Some(0), None   ], // x=0
        vec![None,    None,    None   ], // x=1
        vec![None,    None  ,  None   ], // x=2
        vec![None,    Some(2), None   ], // x=3 
    ];
    let pin2coor = derive_pin2coor(&coor2pin, nx, ny, n_pin);
    #[rustfmt::skip]
    let a = Placement { problem: &problem, coor2pin, pin2coor, _cost:None };

    #[rustfmt::skip]
    let coor2pin = vec![
        //   y=0      y=1      y=2
        vec![None,    None,    None   ], // x=0
        vec![None,    None,    Some(0)], // x=1
        vec![None,    None,    None   ], // x=2
        vec![None,    Some(1), Some(2)], // x=3
    ];
    let pin2coor = derive_pin2coor(&coor2pin, nx, ny, n_pin);
    #[rustfmt::skip]
    let b = Placement { problem: &problem, coor2pin, pin2coor, _cost:None };

    let mut out = b.clone();
    crossover_half(&a, &b, &mut out, 2, &mut rand::thread_rng());

    print_coor2pin(&problem, &out.coor2pin);
    assert_eq!(&out.coor2pin[0][1], &Some(0));
    assert_eq!(&out.coor2pin[3][1], &Some(1));
    assert_eq!(&out.coor2pin[3][2], &Some(2));
}

#[allow(dead_code)]
fn print_coor2pin(problem: &Problem, coor2pin: &Vec<Vec<Option<PinID>>>) {
    println!("==============");
    for x in 0..problem.nx {
        for y in 0..problem.ny {
            print!("{:10} ", format!("{:?}", &coor2pin[x][y]));
        }
        println!("");
    }
}

fn print_stats(i_iter: &usize, population: &mut Vec<Placement>) {
    let best = population[0].cost_panic();
    let mean = population
        .iter_mut()
        .map(|p| p.cost_mut() as f32)
        .sum::<f32>()
        / population.len() as f32;
    let variance = population
        .iter()
        .map(|p| {
            let diff = mean - p.cost_panic() as f32;
            diff * diff
        })
        .sum::<f32>()
        / population.len() as f32;
    let std = variance.sqrt();

    println!(
        "@ i={i:7} | best={best:6} | mean={mean:6.0} | std={std:4.0} | n_pop={n_pop:4}",
        i = i_iter,
        best = best,
        mean = mean,
        std = std,
        n_pop = population.len()
    );
    println!("{}", serde_json::to_string(&population[0]).unwrap());
}

fn last_n_equal(vec: &Vec<usize>, n: usize) -> bool {
    if n < 2 {
        panic!("n should >= 2");
    }

    if vec.len() < n {
        return false;
    }

    let last = *vec.last().unwrap();
    for i in (vec.len() - n)..(vec.len() - 1) {
        if vec[i] != last {
            return false;
        }
    }
    return true;
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
    let mut costs = Vec::new();

    let rng = &mut rand::thread_rng();
    loop {
        // compute fitness and sort
        population.iter_mut().for_each(|p| {
            p.cost_mut();
        });
        population.sort_by_cached_key(|i| i.cost_panic());
        if i_iter % 1000 == 0 {
            print_stats(&i_iter, &mut population);
            costs.push(population.first().unwrap().cost_panic());
            if last_n_equal(&costs, 10) {
                break;
            }
        }
        // break if converge
        if i_iter > params.n_generation {
            break;
        } else {
            i_iter += 1;
        }
        // selection
        let selection_base = population.split_off(params.n_elite);
        let elite = population;
        // FIXME: pass padding conditions there
        let survived = selection(selection_base, params.n_select - params.n_elite, true);

        // FPGA PLACEMENT OPTIMIZATION BY TWO-STEP UNIFIED GENETIC ALGORITHM AND SIMULATED ANNEALING ALGORITHM
        // crossover
        let mut crossed = Vec::new();
        let parents: Vec<&Placement> = elite.iter().chain(survived.iter()).collect();
        for _ in 0..params.n_crossover {
            let i_pa = rng.gen_range(0..parents.len());
            let i_pb = rng.gen_range(0..parents.len());
            let a = parents[i_pa];
            let b = parents[i_pb];
            // TODO: add lifetime and clone the placement in crossover_half
            let mut c = a.clone();
            let mut d = b.clone();
            c._cost = None;
            d._cost = None;
            crossover(a, b, &mut c, &mut d);
            crossed.push(c);
            crossed.push(d);
        }

        // mutation
        let mut mutation_base = survived;
        mutation_base.extend(crossed.into_iter());
        for i in 0..mutation_base.len() {
            if rng.gen::<f32>() < params.p_mutation {
                mutate(&mut mutation_base[i], &i_iter);
            }
        }

        // join population
        population = elite;
        population.extend(mutation_base);

        // // local improvement
        // if let Some(p) = improve(population.choose(rng).unwrap().clone()) {
        //     population.push(p);
        // }
    }
}
