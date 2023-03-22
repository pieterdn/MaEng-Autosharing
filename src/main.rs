mod model;
mod input;
mod output;
use crate::input::read_input;
use crate::model::{
    Request,
    Zone,
    Solution
};
use crate::output::ouput_solution;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::iter::zip;
use rand::seq::{
    IteratorRandom, SliceRandom
};

fn create_initial_input<'a>(reqs: &'a Vec<Request>,
                        zones: &'a Vec<Zone>,
                        amount_cars: i64,
                        rng: &mut rand::rngs::StdRng) -> Solution<'a> {
    let mut reqsol = Solution::new(reqs.len() as i64, amount_cars, reqs, zones);
    let mut rand_reqs = reqs.iter().choose_multiple(rng, reqs.len());
    rand_reqs.shuffle(rng);
    for req in rand_reqs {
        let i = req.req;
        for &car in req.cars.iter().choose_multiple(rng, req.cars.len()) {
            let zone = reqsol.car_to_zone[car as usize];
            if zone < 0 {
                let new_zone = req.zone;
                reqsol.change_cost(i as i64, 0);
                reqsol.zone_hard_change(car, new_zone);
                reqsol.car_hard_change(i as i64, car);
                break;
            } else {
                if reqsol.feasible_car_to_req(i as i64, car) {
                    let (new_cost , _) = reqsol.cost_and_feasible_zone(i as i64, zone);
                    reqsol.change_cost(i as i64, new_cost);
                    reqsol.car_hard_change(i as i64, car);
                    break;
                }
            }
        }
    }
    for i in 0..reqsol.car_to_zone.len() {
        let zone = reqsol.car_to_zone[i];
        if zone < 0 {
            reqsol.car_to_zone[i] = 0;
        }
    }
    return reqsol;
}


fn main() -> Result<(), String>{
    let (input, ouput, time, seed, threads) = parse_args(env::args())?;
    let mut rng = rand::SeedableRng::seed_from_u64(seed);
    let mut file = File::open(input).map_err(|x| format!("io error: {x}"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|x| format!("io error: {x}"))?;
    let (reqs, zones, vehicles_amount) = read_input(contents).unwrap();
    let reqsol = create_initial_input(&reqs, &zones, vehicles_amount, &mut rng);
    let best_sol = reqsol.to_model();
    println!("{:?}", reqs);
    println!("{:?}", zones);
    println!("{:?}", vehicles_amount);
    // println!("{:?}", best_sol);
    ouput_solution(ouput, best_sol)?;
    return Ok(());
}


fn parse_args(mut args: env::Args) -> Result<(String, String, i32, u64, i32), String> {
    let mut error = String::new();
    let mut input: String = String::new();
    let mut output: String = String::new();
    let mut time: i32 = 0;
    let mut seed: u64 = 0;
    let mut threads: i32 = 0;
    args.next();
    if let Some(_in) = args.next() {
        input = _in;
    } else {
        error.push_str("Missing input file arg\n");
    }
    if let Some(_out) = args.next() {
        output = _out;
    } else {
        error.push_str("Missing ouput file arg\n");
    }
    if let Some(_time) = args.next() {
        if let Ok(num) = str::parse(&_time) {
            time = num;
        } else {
            error.push_str(&format!("Unable to parse {_time} to int\n"));
        }
    } else {
        error.push_str("Missing time arg\n");
    }
    if let Some(_seed) = args.next() {
        if let Ok(num) = str::parse(&_seed) {
            seed = num;
        } else {
            error.push_str(&format!("Unable to parse {_seed} to int\n"));
        }
    } else {
        error.push_str("Missing seed arg\n");
    }
    if let Some(_threads) = args.next() {
        if let Ok(num) = str::parse(&_threads) {
            threads = num;
        } else {
            error.push_str(&format!("Unable to parse {_threads} to int\n"));
        }
    } else {
        error.push_str("Missing amount of threads arg\n");
    }

    if error.is_empty() {
        return Ok((input, output, time, seed, threads));
    } else {
        return Err(error);
    }
}
