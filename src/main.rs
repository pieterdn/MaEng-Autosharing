mod model;
mod input;
use crate::input::read_input;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file = File::open("input_files/toy1.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (reqs, zones, vehicles_amount) = read_input(contents).unwrap();
    println!("{:?}", reqs);
    println!("{:?}", zones);
    println!("{:?}", vehicles_amount);
    return Ok(());
}
