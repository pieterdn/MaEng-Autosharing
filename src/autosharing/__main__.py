from __future__ import annotations
from typing import List, Tuple
from autosharing.model import RequestStruct, Solution, ZoneStruct
from autosharing.input import ProcessInput
from autosharing.output import ProcessOutput
import time
import argparse
import random
from threading import Timer

def create_initial_input(reqs: List[RequestStruct],
                         zones: List[ZoneStruct],
                         amount_cars: int) -> Solution:
    reqsol = Solution(len(reqs), amount_cars, reqs, zones)
    for i, req in enumerate(reqs):
        car: int
        for car in req.cars:
            zone = reqsol.car_to_zone[car]
            if zone < 0:
                new_zone = req.zone
                reqsol.changeCost(i, 0)
                reqsol.zoneHardChange(car, new_zone)
                reqsol.carHardChange(i, car)
                break
            else:
                if not reqsol.feasibleCarToReq(i, car):
                    continue
                new_cost, _ = reqsol.costAndFeasibleZone(i, zone)
                reqsol.changeCost(i, new_cost)
                reqsol.carHardChange(i, car)
                break
    for i, zone in enumerate(reqsol.car_to_zone):
        if zone < 0:
            reqsol.car_to_zone[i] = 0
    return reqsol


def small_operator(reqsol: Solution, reqs_ints: range, cars_ints: range) -> bool:
    rand_reqs = random.sample(reqs_ints, k=len(reqs_ints))
    rand_cars = random.sample(cars_ints, k=len(cars_ints))
    # req, car, cost
    best: Tuple[int, int, int] | None = None
    # loop only over cars in feasible zone not all
    for req in rand_reqs:
        for car in rand_cars:
            if not reqsol.feasibleCarToReq(req, car):
                continue
            new_cost = reqsol.newCost(req, car)
            if best is None or new_cost < best[2]:
                best = (req, car, new_cost)
    if best is None or best[2] > reqsol.cost:
        return False
    reqsol.addCarToReq(best[0], best[1])
    return True

def big_operator(reqsol: Solution, reqs_ints: range, cars_int: range) -> bool:
    rand_zones = random.sample(range(0, len(reqsol.zones)), k=len(reqsol.zones))
    rand_cars = random.sample(cars_ints, k=len(cars_ints))
    # car, zone, cost
    best: Tuple[int, int, int] | None = None
    for rand_car in rand_cars:
        for rand_zone in rand_zones:
            reqsol.startTransaction()
            big_op(reqsol, cars_int, rand_car, rand_zone)
            if best is None or reqsol.cost < best[2]:
                best = (rand_car, rand_zone, reqsol.cost)
            reqsol.rollback()
    if best is None or best[2] >= reqsol.cost:
        return False
    big_op(reqsol, cars_int, best[0], best[1])
    return True

def big_op(new_reqsol: Solution, cars_int: range, rand_car: int, rand_zone: int):
    lost_before = [i for i, car in enumerate(new_reqsol.req_to_car) if car < 0]
    lost = new_reqsol.changeCarZone(rand_car, rand_zone)
    for req in lost_before:
        if new_reqsol.feasibleCarToReq(req, rand_car):
            new_reqsol.addCarToReq(req, rand_car)
    for req in lost:
        for car in cars_int:
            if new_reqsol.feasibleCarToReq(req, car):
                new_reqsol.addCarToReq(req, car)


end = False
def end_of_calc():
    global end
    end = True


if __name__ == "__main__":

    parser = argparse.ArgumentParser(description="Finding a good solution to the autosharing problem.")
    parser.add_argument('input_file')
    parser.add_argument('output_file')
    parser.add_argument('time_limit_s', type=int)
    parser.add_argument('random_seed', type=int)
    parser.add_argument('thread_amount', type=int)
    argumentNamespace = parser.parse_args()
    # init seed


    #Read input file and create model
    pi = ProcessInput(argumentNamespace.input_file)
    #Create initial solution
    reqsol = create_initial_input(pi.requests, pi.zones, pi.caramount)
    initial_cost = reqsol.cost
    best_sol = reqsol.toModel()
    reqs_ints = range(0, len(pi.requests))
    cars_ints = range(0, pi.caramount)
    zone_ints = range(0, len(pi.zones))
    random.seed(argumentNamespace.random_seed)

    #---------------Start of timing window---------------
    start_time = time.perf_counter()
    Timer(argumentNamespace.time_limit_s, end_of_calc).start()

    while not end:
        if reqsol.cost < best_sol.cost:
            best_sol = reqsol.toModel()
        if not big_operator(reqsol, reqs_ints, cars_ints):
            small_operator(reqsol, reqs_ints, cars_ints)
        # if not small_operator(reqsol, reqs_ints, cars_ints):
        #     big_operator(reqsol, reqs_ints, cars_ints)

    elapsed_time = time.perf_counter() - start_time
    #----------------End of timing window----------------

    print(f"Elapsed time: {elapsed_time}s")
    print(f"Cost improvement: {initial_cost} -> {best_sol.cost}")
    ProcessOutput(argumentNamespace.output_file, best_sol)
