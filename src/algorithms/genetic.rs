use rand::seq::SliceRandom;

use crate::blif::BLIFInfo;
use crate::placement::Placement;
// use crate::typing::Pin;
use crate::typing::{Coor, Net, Pin};

fn mutate(mut assignment: Placement, coors: &Vec<Coor>, n_mutation: usize) -> Placement {
    for _ in 0..n_mutation {
        let ab = coors
            .choose_multiple(&mut rand::thread_rng(), 2)
            .collect::<Vec<_>>();
        let (ca, cb) = (*ab[0], *ab[1]);

        assignment.swap(ca, cb);
    }
    assignment
}

pub fn genetic_placement(
    blif: &BLIFInfo,
    nx: usize,
    ny: usize,
    n_population: usize,
    n_parent: usize,
    n_child_per_parent: usize,
    n_mutation_per_child: usize,
) {
    let mut coors: Vec<Coor> = Vec::new();
    for x in 0..nx {
        for y in 0..ny {
            coors.push((x, y));
        }
    }

    let mut nets: Vec<Net> = Vec::new();
    let mut i_net = 0;
    for (name, pins) in blif.net_list.iter() {
        nets.push(Net {
            id: i_net,
            name: name.clone(),
            pins: pins.clone(),
        });
        i_net += 1;
    }

    let mut pins: Vec<Pin> = Vec::new();
    for i_pin in 0..blif.n_pin {
        pins.push(Pin {
            id: i_pin,
            net_ids: Vec::new(),
        });
    }
    for net in nets.iter() {
        for pin_id in &net.pins {
            pins[*pin_id].net_ids.push(net.id);
        }
    }

    // println!("{:?}", cell_assignment);

    let sol = Placement::new(nx, ny, &coors, &blif);
    // to calculate half-perimeter cost
    println!("{:?}", sol.cost(&nets));

    // init population
    let mut population: Vec<Placement> = Vec::new();
    for _ in 0..n_population {
        let sol = Placement::new(nx, ny, &coors, &blif);
        println!("{}", sol.cost(&nets));
        population.push(sol);
    }

    let mut i_iter = 0;
    let mut is_converged = || -> bool {
        if i_iter > 100000 {
            true
        } else {
            i_iter += 1;
            false
        }
    };

    loop {
        // compute fitness and sort
        population.sort_by_key(|i| i.cost(&nets));
        println!("{}", population[0].cost(&nets));
        // break if converge
        if is_converged() {
            break;
        }
        // selection
        // population.drain(n_parent..);
        population.drain(n_population..);
        let parents = &population[..n_parent];
        let mut offsprings: Vec<Placement> = Vec::new();

        // Revisiting Genetic Algorithms for the FPGA Placement Problem
        // crossover: Nope

        // mutation
        for parent in parents {
            for _ in 0..n_child_per_parent {
                let mutated = mutate((*parent).clone(), &coors, n_mutation_per_child);
                offsprings.push(mutated);
            }
        }
        population.extend(offsprings);
    }
}
