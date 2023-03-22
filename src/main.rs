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
use rand::seq::{
    IteratorRandom, SliceRandom
};
use tokio::task;
use tokio::time::{
    sleep,
    Duration
};
use std::time::Instant;

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

fn small_operator(reqsol: &mut Solution,
                  req_ints: &mut Vec<i64>,
                  cars_ints: &mut Vec<i64>,
                  rng: &mut rand::rngs::StdRng) -> bool {
    req_ints.shuffle(rng);
    cars_ints.shuffle(rng);
    // req, car, cost
    let mut best: Option<(i64, i64, i64)> = None;
    for &req in req_ints.iter() {
        for &car in cars_ints.iter() {
            if !reqsol.feasible_car_to_req(req, car) { continue; }
            let new_cost = reqsol.new_cost(req, car);
            if None == best || new_cost < best.unwrap().2 {
                best = Some((req, car, new_cost));
            }
        }
    }
    if best == None || best.unwrap().2 >= reqsol.cost {
        // println!("\tSmall operator failed improvement: {:}", reqsol.cost);
        return false;
    }

    let best = best.unwrap();
    reqsol.add_car_to_req(best.0, best.1);
    // println!("\tSmall operator succeeded improvement: {:}", reqsol.cost);
    return true;
}

fn big_operator(reqsol: &mut Solution,
                zone_ints: &mut Vec<i64>,
                cars_ints: &mut Vec<i64>,
                rng: &mut rand::rngs::StdRng) -> bool {
    zone_ints.shuffle(rng);
    cars_ints.shuffle(rng);
    let old_cost = reqsol.cost;
    for &car in cars_ints.iter() {
        for &zone in zone_ints.iter() {
            reqsol.start_transaction();
            big_op(reqsol, cars_ints, car, zone);
            if reqsol.cost < old_cost {
                reqsol.commit();
                // println!("\tBig operator succeeded improvement: {:}", reqsol.cost);
                return true;
            }
            reqsol.rollback();
        }
    }
    // println!("\tBig operator failed improvement: {:}", reqsol.cost);
    return false;
}

fn big_op(reqsol: &mut Solution,
          cars_ints: &Vec<i64>,
          rand_car: i64,
          rand_zone: i64) {
    let mut lost_before = Vec::new();
    let lost = reqsol.change_car_zone(rand_car, rand_zone);
    for (i, &car) in reqsol.req_to_car.iter().enumerate() {
        if car < 0 {
            lost_before.push(i);
        }
    }
    for &req in lost_before.iter() {
        let req = req as i64;
        if reqsol.feasible_car_to_req(req, rand_car) {
            reqsol.add_car_to_req(req, rand_car);
        }
    }
    for &req in lost.iter() {
        for &car in cars_ints.iter() {
            if reqsol.feasible_car_to_req(req, car) {
                reqsol.add_car_to_req(req, car);
            }
        }
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), String>{
    let (input, ouput, time, seed, threads) = parse_args(env::args())?;
    let join = task::spawn(async move {
        sleep(Duration::from_secs(time as u64)).await;
    });
    let mut rng = rand::SeedableRng::seed_from_u64(seed);
    let mut file = File::open(input).map_err(|x| format!("io error: {x}"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|x| format!("io error: {x}"))?;
    let (reqs, zones, vehicles_amount) = read_input(contents).unwrap();
    let mut reqsol = create_initial_input(&reqs, &zones, vehicles_amount, &mut rng);
    let mut best_sol = reqsol.to_model();
    let mut zone_ints: Vec<i64> = (0..reqsol.zones.len()).map(|x| x as i64).collect();
    zone_ints.shuffle(&mut rng);
    let mut cars_ints: Vec<i64> = (0..vehicles_amount).map(|x| x as i64).collect();
    cars_ints.shuffle(&mut rng);
    let mut req_ints: Vec<i64> = (0..reqsol.reqs.len()).map(|x| x as i64).collect();
    zone_ints.shuffle(&mut rng);
    let start = Instant::now();

    let mut count = 0;
    let mut once = false;
    let mut initial_cost = reqsol.cost;
    let mut initial_best = reqsol.cost;
    while !join.is_finished(){
        if !big_operator(&mut reqsol, &mut zone_ints, &mut cars_ints, &mut rng) {
            if count > 1 {
                while small_operator(&mut reqsol, &mut req_ints, &mut cars_ints, &mut rng) {}
                if once {
                    once = false;
                } else {
                    once = true;
                    // println!("\tCost improvement: {:} -> {:}", initial_cost, reqsol.cost);
                    reqsol = create_initial_input(&reqs, &zones, vehicles_amount, &mut rng);
                    initial_cost = reqsol.cost;
                } 
            } else {
                for _ in 0..5 {
                    if !small_operator(&mut reqsol, &mut req_ints, &mut cars_ints, &mut rng) {
                        break;
                    }
                }
            }
            count += 1;
        }
        if reqsol.cost < best_sol.cost {
            initial_best = initial_cost;
            best_sol = reqsol.to_model();
        }
    }
    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration);
    println!("Cost improvement: {:} -> {:}", initial_best, best_sol.cost);
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
