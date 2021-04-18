use super::util::take_2;
use crate::problem::Problem;

pub struct Params {
    pub t_init: f32,
    pub t_decrease_factor: f32,
    pub t_terminate: f32,
}

pub fn annealing_placement(problem: &Problem, params: &Params) {
    let mut t = params.t_init;
    let mut i_iter = 0;
    let n_batch = (100_f32 * (problem.n_pin as f32).powf(4. / 3.)) as usize;

    let mut sol = problem.make_placement();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    loop {
        let mut acc_delta: i128 = 0;
        for _ in 0..n_batch {
            // randomly select two pins
            let (ca, cb) = take_2(&problem.coors);
            // calculate previous cost
            let cost_prev = sol.cell_cost(ca) + sol.cell_cost(cb);
            // swap pin position
            sol.swap(ca, cb);
            // calculate current cost
            let cost_curr = sol.cell_cost(ca) + sol.cell_cost(cb);
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
            sol.cost_mut()
        );

        // decrease t
        t *= params.t_decrease_factor;
        if t < params.t_terminate {
            break;
        }
        i_iter += 1;
    }
}
