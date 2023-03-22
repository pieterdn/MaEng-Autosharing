use std::collections::HashSet;
use std::cmp::{
    max,
    min
};
use std::cell::RefCell;

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
    carFrom: i64,
    carTo: i64,
    costFrom: i64,
    costTo: i64
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
    req_to_car: Vec<i64>,
    req_to_car_bools: Vec<Vec<bool>>,
    car_to_req_number: Vec<HashSet<i64>>,
    car_to_zone_bools: Vec<Vec<bool>>,
    car_to_zone: Vec<i64>,
    reqs: &'a Vec<Request>,
    zones: &'a Vec<Zone>,
    cost_per_req: Vec<i64>,
    cost: i64,
    in_trans: bool,
    transaction: Vec<Transaction>
}

impl<'a> Solution<'a> {
    fn new(num_reqs: i64,
           num_cars: i64,
           reqs: &'a Vec<Request>,
           zones: &'a Vec<Zone>) -> Solution<'a> {
        let mut cost = 0;
        let mut cost_per_req = Vec::with_capacity(num_cars as usize);
        for (i, req) in reqs.iter().enumerate() {
            cost_per_req[i] = req.pen1;
            cost += req.pen1;
        }
        Solution {
            req_to_car: (0..num_reqs).map(|x| -1).collect(),
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

    fn start_transaction(&mut self) {
        self.in_trans = true;
    }

    fn commit(&mut self) {
        self.in_trans = false;
        self.transaction.clear();
    }

    fn rollback(&mut self) {
        self.in_trans = false;
        for trans in self.transaction.iter().rev() {
            match trans {
                Transaction::Req(trans_req) => {
                    self.req_to_car[trans_req.req as usize] = trans_req.carFrom;
                    if trans_req.carTo >= 0 {
                        self.req_to_car_bools
                            [trans_req.req as usize]
                            [trans_req.carTo as usize]
                                = false;
                        self.car_to_req_number[trans_req.carTo as usize].remove(&trans_req.req);
                    }
                    if trans_req.carFrom >= 0 {
                        self.req_to_car_bools
                            [trans_req.req as usize]
                            [trans_req.carFrom as usize]
                                = true;
                        self.car_to_req_number[trans_req.carFrom as usize]
                            .insert(trans_req.req);
                    }
                    self.cost_per_req[trans_req.req as usize] = trans_req.costFrom;
                    self.cost += trans_req.costFrom - trans_req.costTo;
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

    fn to_model(&self) -> SolutionModel<'a> {
        SolutionModel {
            req_to_car: self.req_to_car.to_vec(),
            car_to_zone: self.car_to_zone.to_vec(),
            reqs: self.reqs,
            zones: self.zones,
            cost: self.cost
        }
    }

    fn cost_and_feasible_zone(&self, req: i64, zone: i64) -> (i64, bool) {
        let req_struct = &self.reqs[req as usize];
        if self.zones[zone as usize].zonerel[req_struct.zone as usize] {
            return (req_struct.pen2, true);
        } else if zone == req_struct.zone {
            return (0, true);
        }
        return (req_struct.pen1, false);
    }

    fn feasible_car_to_req(&self, req: i64, car: i64) -> bool {
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

    fn cost_of_car(&self, req: i64, car: i64) -> (i64, bool) {
        let zone_car = self.car_to_zone[car as usize];
        return self.cost_and_feasible_zone(req, zone_car)
    }

    fn change_cost(&mut self, req: i64, new_cost: i64) {
        let old_cost = self.cost_per_req[req as usize];
        self.cost_per_req[req as usize] = new_cost;
        self.cost += new_cost - old_cost;
    }

    fn new_cost(&self, req: i64, car: i64) -> i64 {
        let (new_cost, _) = self.cost_of_car(req, car);
        let old_cost = self.cost_per_req[req as usize];
        return self.cost + new_cost - old_cost;
    }

    fn zone_hard_change(&mut self, car: i64, zone: i64) {
        let old_zone = self.car_to_zone[car as usize];
        self.car_to_zone_bools
            [old_zone as usize]
            [car as usize]
                = false;
        if zone >= 0 {
            self.car_to_zone[car as usize] = zone;
            self.car_to_zone_bools
                [zone as usize]
                [car as usize]
                    = true;
        }
    }

    fn car_hard_change(&mut self, req: i64, car: i64) {
        let old_car = self.req_to_car[req as usize];
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
                [old_car as usize]
                    = true;
            self.car_to_req_number[car as usize].insert(req);
        }
    }

    fn add_car_to_req(&mut self, req: i64, car: i64) {
        let (new_cost, _) = self.cost_of_car(req, car);
        if self.in_trans {
            self.transaction.push(
                Transaction::Req(TransactionReq {
                    req,
                    carFrom: self.req_to_car[req as usize],
                    carTo: car,
                    costFrom: self.cost_per_req[req as usize],
                    costTo: new_cost 
                })
                );
        }
        self.car_hard_change(req, car);
        self.change_cost(req, new_cost);
    }

    fn change_car_zone(&mut self, car: i64, zone: i64) -> Vec<i64> {
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
                                carFrom: car,
                                carTo: car,
                                costFrom: self.cost_per_req[req as usize],
                                costTo: new_cost 
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
                                carFrom: car,
                                carTo: -1,
                                costFrom: self.cost_per_req[req as usize],
                                costTo: new_cost 
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
                                zone_from: zone,
                                zone_to: self.car_to_zone[car as usize],
                            })
                        );
        }
        self.zone_hard_change(car, zone);
        return lost_items;
    }
}

#[derive(Debug)]
pub struct Request {
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
    pub zonerel: Vec<bool>,
    pub nextto: Vec<i64>,
}
