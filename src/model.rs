use std::collections::HashSet;
use std::cmp::{
    max,
    min
};

const MINUTES_IN_DAY: i64 = 1440;

#[derive(Debug)]
pub struct SolutionModel<'a> {
    pub req_to_car: Vec<i64>,
    pub car_to_zone: Vec<i64>,
    pub reqs: &'a Vec<Request>,
    pub zones: &'a Vec<Zone>,
    pub cost: i64
}

#[derive(Debug)]
struct TransactionReq {
    req: i64,
    car_from: i64,
    car_to: i64,
    cost_from: i64,
    cost_to: i64
}

#[derive(Debug)]
struct TransactionCar {
    car: i64,
    zone_from: i64,
    zone_to: i64,
}

#[derive(Debug)]
enum Transaction {
    Req(TransactionReq),
    Car(TransactionCar)
}

#[derive(Debug)]
pub struct Solution<'a> {
    pub req_to_car: Vec<i64>,
    pub req_to_car_bools: Vec<Vec<bool>>,
    pub car_to_req_number: Vec<HashSet<i64>>,
    pub car_to_zone_bools: Vec<Vec<bool>>,
    pub car_to_zone: Vec<i64>,
    pub reqs: &'a Vec<Request>,
    pub zones: &'a Vec<Zone>,
    pub cost_per_req: Vec<i64>,
    pub cost: i64,
    in_trans: bool,
    transaction: Vec<Transaction>
}

impl<'a> Solution<'a> {
    pub fn new(num_reqs: i64,
           num_cars: i64,
           reqs: &'a Vec<Request>,
           zones: &'a Vec<Zone>) -> Solution<'a> {
        let mut cost = 0;
        let mut cost_per_req = vec![0; num_reqs as usize];
        for (i, req) in reqs.iter().enumerate() {
            cost_per_req[i] = req.pen1;
            cost += req.pen1;
        }
        Solution {
            req_to_car: (0..num_reqs).map(|_| -1).collect(),
            req_to_car_bools: vec![vec![false; num_cars as usize]; num_reqs as usize],
            car_to_req_number: vec![HashSet::new(); num_cars as usize],
            car_to_zone_bools: vec![vec![false; num_cars as usize]; zones.len() as usize],
            car_to_zone: vec![-1; num_cars as usize],
            reqs,
            zones,
            cost_per_req,
            cost,
            in_trans: false,
            transaction: Vec::new()
        }
    }

    pub fn start_transaction(&mut self) {
        self.in_trans = true;
    }

    pub fn commit(&mut self) {
        self.in_trans = false;
        self.transaction.clear();
    }

    pub fn rollback(&mut self) {
        self.in_trans = false;
        for trans in self.transaction.iter().rev() {
            match trans {
                Transaction::Req(trans_req) => {
                    self.req_to_car[trans_req.req as usize] = trans_req.car_from;
                    if trans_req.car_to >= 0 {
                        self.req_to_car_bools
                            [trans_req.req as usize]
                            [trans_req.car_to as usize]
                                = false;
                        self.car_to_req_number[trans_req.car_to as usize].remove(&trans_req.req);
                    }
                    if trans_req.car_from >= 0 {
                        self.req_to_car_bools
                            [trans_req.req as usize]
                            [trans_req.car_from as usize]
                                = true;
                        self.car_to_req_number[trans_req.car_from as usize]
                            .insert(trans_req.req);
                    }
                    self.cost_per_req[trans_req.req as usize] = trans_req.cost_from;
                    self.cost += trans_req.cost_from - trans_req.cost_to;
                },
                Transaction::Car(trans_car) => {
                    self.car_to_zone[trans_car.car as usize] = trans_car.zone_from;
                    self.car_to_zone_bools
                        [trans_car.zone_to as usize]
                        [trans_car.car as usize]
                            = false;
                    self.car_to_zone_bools
                        [trans_car.zone_from as usize]
                        [trans_car.car as usize]
                            = true;
                }
            }
        }
        self.transaction.clear();
    }

    pub fn to_model(&self) -> SolutionModel<'a> {
        SolutionModel {
            req_to_car: self.req_to_car.to_vec(),
            car_to_zone: self.car_to_zone.to_vec(),
            reqs: self.reqs,
            zones: self.zones,
            cost: self.cost
        }
    }

    pub fn cost_and_feasible_zone(&self, req: i64, zone: i64) -> (i64, bool) {
        let req_struct = &self.reqs[req as usize];
        if self.zones[zone as usize].zonerel[req_struct.zone as usize] {
            return (req_struct.pen2, true);
        } else if zone == req_struct.zone {
            return (0, true);
        }
        return (req_struct.pen1, false);
    }

    pub fn feasible_car_to_req(&self, req: i64, car: i64) -> bool {
        let zone = self.car_to_zone[car as usize];
        if zone < 0 {
            return false;
        }
        let req_struct = &self.reqs[req as usize];
        let mut ok = false;
        for cr in &req_struct.cars {
            if *cr == car {
                ok = true;
                break;
            }
        }
        if !ok { return false };
        let (_, feasible) = self.cost_and_feasible_zone(req, zone);
        if !feasible {
            return false;
        }
        let req_start = req_struct.day*MINUTES_IN_DAY + req_struct.start;
        let req_end = req_start + req_struct.time;
        for alloc_req in &self.car_to_req_number[car as usize] {
            let areq_struct = &self.reqs[*alloc_req as usize];
            let areq_start = areq_struct.day*MINUTES_IN_DAY + areq_struct.start;
            let areq_end = areq_start + areq_struct.time;
            if !(max(req_start, areq_start)..(min(req_end, areq_end) + 1)).is_empty() {
                return false;
            }
        }
        return true;
    }

    pub fn cost_of_car(&self, req: i64, car: i64) -> (i64, bool) {
        let zone_car = self.car_to_zone[car as usize];
        return self.cost_and_feasible_zone(req, zone_car)
    }

    pub fn change_cost(&mut self, req: i64, new_cost: i64) {
        let old_cost = self.cost_per_req[req as usize];
        self.cost_per_req[req as usize] = new_cost;
        self.cost += new_cost - old_cost;
    }

    pub fn new_cost(&self, req: i64, car: i64) -> i64 {
        let (new_cost, _) = self.cost_of_car(req, car);
        let old_cost = self.cost_per_req[req as usize];
        return self.cost + new_cost - old_cost;
    }

    pub fn zone_hard_change(&mut self, car: i64, zone: i64) {
        let old_zone = self.car_to_zone[car as usize];
        // println!("{:?} {:?}", car, old_zone);
        if old_zone >= 0 {
            self.car_to_zone_bools
                [old_zone as usize]
                [car as usize]
                    = false;
        }
        if zone >= 0 {
            self.car_to_zone[car as usize] = zone;
            self.car_to_zone_bools
                [zone as usize]
                [car as usize]
                    = true;
        }
    }

    pub fn car_hard_change(&mut self, req: i64, car: i64) {
        let old_car = self.req_to_car[req as usize];
        self.req_to_car[req as usize] = car;
        if old_car >= 0 {
            self.req_to_car_bools
                [req as usize]
                [old_car as usize]
                    = false;
            self.car_to_req_number[old_car as usize].remove(&req);
        }
        if car >= 0 {
            self.req_to_car_bools
                [req as usize]
                [car as usize]
                    = true;
            self.car_to_req_number[car as usize].insert(req);
        }
    }

    pub fn add_car_to_req(&mut self, req: i64, car: i64) {
        let (new_cost, _) = self.cost_of_car(req, car);
        if self.in_trans {
            self.transaction.push(
                Transaction::Req(TransactionReq {
                    req,
                    car_from: self.req_to_car[req as usize],
                    car_to: car,
                    cost_from: self.cost_per_req[req as usize],
                    cost_to: new_cost 
                })
                );
        }
        self.car_hard_change(req, car);
        self.change_cost(req, new_cost);
    }

    pub fn change_car_zone(&mut self, car: i64, zone: i64) -> Vec<i64> {
        let mut lost_items: Vec<i64> = Vec::new();
        let mut reqs = Vec::with_capacity(self.car_to_req_number[car as usize].len());
        for &req in &self.car_to_req_number[car as usize] {
            reqs.push(req);
        }
        // make sure self.car_to_req_number does not get changed in for
        for req in reqs {
            let (new_cost, feasible) = self.cost_and_feasible_zone(req, zone);
            if feasible {
                if self.in_trans {
                    self.transaction.push(
                        Transaction::Req(
                            TransactionReq {
                                req,
                                car_from: car,
                                car_to: car,
                                cost_from: self.cost_per_req[req as usize],
                                cost_to: new_cost 
                            })
                        );
                }
                self.change_cost(req, new_cost);
            } else {
                if self.in_trans {
                    self.transaction.push(
                        Transaction::Req(
                            TransactionReq {
                                req,
                                car_from: car,
                                car_to: -1,
                                cost_from: self.cost_per_req[req as usize],
                                cost_to: new_cost 
                            })
                        );
                }
                lost_items.push(req);
                self.change_cost(req, new_cost);
                self.car_hard_change(req, -1);
            }
        }
        if self.in_trans {
            self.transaction.push(
                        Transaction::Car(
                            TransactionCar {
                                car,
                                zone_from: self.car_to_zone[car as usize],
                                zone_to: zone,
                            })
                        );
        }
        self.zone_hard_change(car, zone);
        return lost_items;
    }
}

#[derive(Debug)]
pub struct Request {
    pub req: i64,
    pub zone: i64,
    pub day: i64,
    pub start: i64,
    pub time: i64,
    pub cars: Vec<i64>,
    pub pen1: i64,
    pub pen2: i64,
}

#[derive(Debug)]
pub struct Zone {
    pub zone: i64,
    pub zonerel: Vec<bool>,
    pub nextto: Vec<i64>,
}
