use rand::seq::SliceRandom;

use crate::blif::BLIFInfo;
use crate::placement;
use crate::typing;

pub fn annealing_placement(
    blif: &BLIFInfo,
    nx: usize,
    ny: usize,
    t_init: f32,
    t_decrease_factor: f32,
    t_terminate: f32,
) {
    let mut t = t_init;
    let mut i_iter = 0;
    let n_batch = (100_f32 * (blif.n_pin as f32).powf(4. / 3.)) as usize;

    use rand::Rng;
    let mut rng = rand::thread_rng();

    use typing::Coor;
    let mut coors: Vec<Coor> = Vec::new();
    for x in 0..nx {
        for y in 0..ny {
            coors.push((x, y));
        }
    }

    use placement::Placement;
    let mut sol = Placement::new(nx, ny, &coors, &blif);

    use typing::Net;
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

    use typing::Pin;
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

    loop {
        let mut acc_delta: i128 = 0;
        for _ in 0..n_batch {
            // randomly select two pins
            let (ca, cb) = take_2(&coors);
            // calculate previous cost
            let cost_prev = sol.cell_cost(&pins, &nets, ca) + sol.cell_cost(&pins, &nets, cb);
            // swap pin position
            sol.swap(ca, cb);
            // calculate current cost
            let cost_curr = sol.cell_cost(&pins, &nets, ca) + sol.cell_cost(&pins, &nets, cb);
            // calculate delta cost
            let delta_cost: f32 = cost_curr as f32 - cost_prev as f32;

            let r: f32 = rng.gen();
            if r < f32::exp(-(delta_cost as f32) / t) {
                acc_delta += delta_cost as i128; // confirm swap
            } else {
                sol.swap(ca, cb); // restore swap
            }
        }
        println!(
            "i={:3}   t={:.2}   d_cost={:7}   cost={:5}",
            i_iter,
            t,
            acc_delta,
            sol.cost(&nets)
        );

        // decrease t
        t *= t_decrease_factor;
        if t < t_terminate {
            break;
        }
        i_iter += 1;
    }
}

fn take_2<T>(v: &Vec<T>) -> (T, T)
where
    T: Copy,
{
    let ab = v
        .choose_multiple(&mut rand::thread_rng(), 2)
        .collect::<Vec<_>>();
    (*ab[0], *ab[1])
}
