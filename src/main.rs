mod model;
mod input;
mod output;
use crate::input::read_input;
use std::fs::File;
use std::io::prelude::*;
use std::env;


fn main() -> Result<(), String>{
    let (input, ouput, time, seed, threads) = parse_args(env::args())?;
    let mut file = File::open(input).map_err(|x| format!("io error: {x}"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|x| format!("io error: {x}"))?;
    let (reqs, zones, vehicles_amount) = read_input(contents).unwrap();
    println!("{:?}", reqs);
    println!("{:?}", zones);
    println!("{:?}", vehicles_amount);
    return Ok(());
}


fn parse_args(mut args: env::Args) -> Result<(String, String, i32, i32, i32), String> {
    let mut error = String::new();
    let mut input: String = String::new();
    let mut output: String = String::new();
    let mut time: i32 = 0;
    let mut seed: i32 = 0;
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
