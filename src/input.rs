use crate::model::{
    Request,
    Zone
};

enum InputField {
    Requests,
    Zones,
}

pub fn read_input(input: String) -> Result<(Vec<Request>, Vec<Zone>, i64), String> {
    let mut reqs: Vec<Request> = Vec::new();
    let mut zones: Vec<Zone> = Vec::new();
    let mut vehicle_amount: Option<i64> = None;
    let mut bleep: InputField = InputField::Requests;
    let mut zone_amount: i64 = 0;
    let mut amount = 0;
    for (i, line) in input.lines().enumerate() {
        if amount != 0 {
            match bleep {
                InputField::Requests => {
                    read_request(line, &mut reqs, i)?;
                    amount -= 1;
                },
                InputField::Zones => {
                    read_zone(line, &mut zones, zone_amount, i)?;
                    amount -= 1;
                },
            }
            continue;
        }
        let line = line.trim();
        let mut splitted = line.split(":");
        if let Some(first_part) = splitted.next() {
            match first_part {
                "+Requests" => {
                    amount = get_amount(splitted.next(), i)?;
                    // amount -= 1;
                    bleep = InputField::Requests;
                },
                "+Zones" => {
                    amount = get_amount(splitted.next(), i)?;
                    zone_amount = amount;
                    // amount -=1 ;
                    bleep = InputField::Zones;
                },
                "+Vehicles" => {
                    let _amount = get_amount(splitted.next(), i)?;
                    vehicle_amount = Some(_amount);
                    break;
                },
                "+Days" => {
                },
                _ => return Err(format!("Invalid line: {:}", i))
            }
        }
    }
    if let Some(am) = vehicle_amount {
        return Ok((reqs, zones, am));
    } else {
        return Err(String::from("No"));
    }
}

fn get_amount(amount: Option<&str>, line_amount: usize) -> Result<i64, String> {
    if let Some(amount_str) = amount {
        if let Ok(num) = str::parse(amount_str.trim()) {
            return Ok(num);
        } else {
            return Err(format!("Cannot parse in for amount in line {:}", line_amount));
        }
    } else {
        return Err(format!("Missing amount in line {:}", line_amount));
    }
}

fn read_request(line: &str,
                 reqs: &mut Vec<Request>,
                 line_num: usize) -> Result<(), String> {
    let mut columns = line.split(';');
    let zoneid: i64;
    if let Some(_) = columns.next() {
    } else {
        return Ok(());
    }
    // first
    if let Some(first) = columns.next() {
        let mut index_it = first.split('z');
        _ = index_it.next();
        let index;
        if let Some(_index) = index_it.next() {
            index = _index;
        } else {
            return Err(format!("Not able to parse int for req on line {:}", line_num));
        }
        if let Ok(num) = str::parse(index.trim()) {
            zoneid = num;
        } else {
            return Err(format!("Not able to parse int for req on line {:}", line_num));
        }
    } else {
        // empty line
        return Ok(());
    }
    let day: i32;
    if let Some(second) = columns.next() {
        if let Ok(num) = str::parse(second.trim()) {
            day = num;
        } else {
            return Err(format!("Not able to parse int for day on line {:}", line_num));
        }
    } else {
        return Err(format!("Missing day on line: {:}", line_num));
    }
    let start: i32;
    if let Some(third) = columns.next() {
        if let Ok(num) = str::parse(third.trim()) {
            start = num;
        } else {
            return Err(format!("Not able to parse int for start on line {:}", line_num));
        }
    } else {
        return Err(format!("Missing start on line: {:}", line_num));
    }
    let time: i32;
    if let Some(fourth) = columns.next() {
        if let Ok(num) = str::parse(fourth.trim()) {
            time = num;
        } else {
            return Err(format!("Not able to parse int for time on line {:}", line_num));
        }
    } else {
        return Err(format!("Missing time on line: {:}", line_num));
    }
    let mut cars: Vec<i64> = Vec::new();
    if let Some(fifth) = columns.next() {
        for car in fifth.split(',') {
            let mut index_it = car.split("car");
            _ = index_it.next();
            let index;
            if let Some(_index) = index_it.next(){
                index = _index;
            } else {
                return Err(format!("Not able to parse int for car on line {:}", line_num));
            }
            if let Ok(num) = str::parse(index.trim()) {
                cars.push(num);
            } else {
                return Err(format!("Not able to parse int for car on line {:}", line_num));
            }
        }
    } else {
        return Err(format!("Missing cars on line: {:}", line_num));
    }
    let pen1;
    if let Some(sixth) = columns.next() {
        if let Ok(num) = str::parse(sixth.trim()) {
            pen1 = num;
        } else {
            return Err(format!("Not able to parse int for pen1 on line {:}", line_num));
        }
    } else {
        return Err(format!("Missing pen1 on line: {:}", line_num));
    }
    let pen2;
    if let Some(sixth) = columns.next() {
        if let Ok(num) = str::parse(sixth.trim()) {
            pen2 = num;
        } else {
            return Err(format!("Not able to parse int for pen1 on line {:}", line_num));
        }
    } else {
        return Err(format!("Missing pen1 on line: {:}", line_num));
    }
    reqs.push(Request {
        zone: zoneid,
        day: day as i64,
        start: start as i64,
        time: time as i64,
        cars,
        pen1,
        pen2
    });
    return Ok(());
}

fn read_zone(line: &str,
             zone: &mut Vec<Zone>,
             zone_amount: i64,
             line_num: usize) -> Result<(), String> {
    let mut zone_rel: Vec<bool> = (0..zone_amount).map(|_| false).collect();
    let mut nextto: Vec<i64> = Vec::new();
    let mut columns = line.split(';');
    // let zone_index: i64;
    if let Some(_) = columns.next() {
        // let mut index_it = first.split('z');
        // _ = index_it.next();
        // let index;
        // if let Some(_index) = index_it.next(){
        //     index = _index;
        // } else {
        //     return Err(format!("Not able to parse int for zone on line {:}", line_num));
        // }
        // if let Ok(num) = str::parse(index.trim()) {
        //     zone_index = num;
        // } else {
        //     return Err(format!("Not able to parse int for zone on line {:}", line_num));
        // }
    } else {
        return Ok(());
    }
    if let Some(second) = columns.next() {
        for zone_str in second.split(','){
            let index_opt = zone_str.get(1..2);
            let index;
            if let Some(_index) = index_opt{
                index = _index;
            } else {
                return Err(format!("Not able to parse int for zone on line {:}", line_num));
            }
            if let Ok(num) = str::parse(index.trim()) {
                if num >= zone_amount as i64{
                    return Err(format!("zone index higher than specified on line {:}", line_num));
                }
                zone_rel[num as usize] = true;
                nextto.push(num);
            } else {
                return Err(format!("Not able to parse int for zone on line {:}", line_num));
            }
        }
    }
    zone.push(Zone {
        zonerel: zone_rel,
        nextto
    });
    return Ok(());
}
