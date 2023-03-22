use crate::model::SolutionModel;
use std::fs;

pub fn ouput_solution(output_file: String,
                  solution: SolutionModel) -> Result<(), String> {
    let mut output = String::new();
    let cost = solution.cost;
    output.push_str(&format!("{cost}\n"));
    output.push_str("+Vehicle assignments\n");
    for i in 0..solution.car_to_zone.len() {
        let zone = solution.car_to_zone[i];
        output.push_str(&format!("car{i};z{zone}\n"));
    }
    output.push_str("+Assigned requests\n");
    for i in 0..solution.req_to_car.len() {
        if solution.req_to_car[i] != -1 {
            let car = solution.req_to_car[i];
            output.push_str(&format!("req{i};car{car}\n"));
        }
    }
    output.push_str("+Unassigned requests\n");
    for i in 0..solution.req_to_car.len() {
        if solution.req_to_car[i] == -1 {
            output.push_str(&format!("req{i}\n"));
        }
    }
    fs::write(output_file, output)
        .map_err(|x| format!("io error writing solution: {x}"))?;
    return Ok(());
}
