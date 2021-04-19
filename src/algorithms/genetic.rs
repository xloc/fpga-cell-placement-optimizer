use rand::seq::SliceRandom;
use rand::Rng;

use crate::placement::Placement;
use crate::problem::Problem;
use crate::typing::{Coor, PinID};

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
    let fitness_sum = fitnesses.iter().fold(0, |acc, (_, fit)| acc + fit);
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

#[allow(dead_code)]
fn make_fixture() -> Problem {
    let filename = "benchmarks/alu2.blif";
    use crate::blif::BLIFInfo;
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

    // copy b.right to out.right
    let mut out_p2c: Vec<Option<Coor>> = vec![None; problem.n_pin];
    for x in i_divide..problem.nx {
        for y in 0..problem.ny {
            if let Some(pin) = b.coor2pin[x][y] {
                out_p2c[pin] = Some((x, y));
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
                } else {
                    // ^ no duplication
                    out.coor2pin[x][y] = a.coor2pin[x][y];
                    out_p2c[pin] = Some((x, y));
                }
            } else {
                out.coor2pin[x][y] = None;
            }
        }
    }

    for (pin_id, _) in out_p2c.iter().enumerate().filter(|(_, p)| p.is_none()) {
        // println!("{}", pin_id);
        loop {
            // FIXME it can be highly inefficient when empty cell is limited
            let (x, y) = *problem.coors.choose(rng).unwrap();
            if out.coor2pin[x][y].is_none() {
                out.coor2pin[x][y] = Some(pin_id);
                break;
            }
        }
    }

    let mut d_new_pin2coor: Vec<Option<Coor>> = vec![None; problem.n_pin];
    for x in 0..problem.nx {
        for y in 0..problem.ny {
            if let Some(pin) = out.coor2pin[x][y] {
                d_new_pin2coor[pin] = Some((x, y));
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
                d_new_pin2coor[pin] = Some((x, y));
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

    use crate::problem::make_coors;
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

    use crate::problem::make_coors;
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
    assert_eq!(&out.coor2pin[0][1], &Some(1));
}

#[test]
fn should_crossover_if_a_left_b_right_have_overlapped_pins() {
    let (nx, ny) = (4, 3);
    let n_pin = 3;

    use crate::problem::make_coors;
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
            println!("{},", population[0].cost_mut());
        }
        // break if converge
        if i_iter > params.n_generation {
            break;
        } else {
            i_iter += 1;
        }
        // selection
        let survived = selection(population, n_reserve, n_random_reserve);

        // FPGA PLACEMENT OPTIMIZATION BY TWO-STEP UNIFIED GENETIC ALGORITHM AND SIMULATED ANNEALING ALGORITHM
        // crossover
        let mut crossed = Vec::new();
        let parents = &survived;
        let mut parent_index: Vec<usize> = (0..parents.len()).collect();
        parent_index.shuffle(rng);
        let mut parents_index_iter = parent_index.iter();
        for _ in 0..parents.len() / 2 {
            if rng.gen::<f32>() < params.crossover_probability {
                let a = &parents[*parents_index_iter.next().unwrap()];
                let b = &parents[*parents_index_iter.next().unwrap()];
                // TODO: add lifetime and clone the placement in crossover_half
                let mut c = a.clone();
                let mut d = b.clone();
                crossover(a, b, &mut c, &mut d);
                crossed.push(c);
                crossed.push(d);
            }
        }

        // mutation
        let mut mutated = Vec::new();
        for p in survived.iter() {
            if rng.gen::<f32>() < params.mutation_probability {
                let mp = mutate(p.clone());
                mutated.push(mp);
            }
        }

        // improvement
        let improved = improve(survived.choose(rng).unwrap().clone());

        // improvement and join population
        population = survived;
        population.extend(mutated);
        population.extend(crossed);
        if let Some(p) = improved {
            population.push(p);
        }
    }
}
