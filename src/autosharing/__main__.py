from __future__ import annotations
from typing import List
from autosharing.model import RequestStruct, Solution, ZoneStruct
from autosharing.input import ProcessInput
from autosharing.output import ProcessOutput
import sys
import time
import argparse

def create_initial_input(reqs: List[RequestStruct],
                         zones: List[ZoneStruct],
                         amount_cars: int) -> Solution:
    #print(len(reqs), amount_cars)
    reqsol = Solution(len(reqs), amount_cars, reqs, zones)
    for i, req in enumerate(reqs):
        # print(reqsol.req_to_car)
        # print(reqsol.car_to_zone)
        for car in req.cars:
            # print(f"car: {car} req: {req}")
            zone = reqsol.car_to_zone[car]
            # print(f"zone{zone}")
            if zone < 0:
                # print("here1")
                new_zone = req.zone
                reqsol.changeCost(i, 0)
                # print(f"car: {car} zone: {new_zone}")
                reqsol.zoneHardChange(car, new_zone)
                reqsol.carHardChange(i, car)
                break
            else:
                #print("here2")
                if not reqsol.feasibleCarToReq(i, car):
                    #print(f"not feasible: req:{i} car:{car}")
                    continue
                new_cost, _ = reqsol.costAndFeasibleZone(i, zone)
                # print(f"req: {i}")
                reqsol.changeCost(i, new_cost)
                reqsol.carHardChange(i, car)
                break
    for i, zone in enumerate(reqsol.car_to_zone):
        if zone < 0:
            reqsol.car_to_zone[i] = 0
    return reqsol



if __name__ == "__main__":

    parser = argparse.ArgumentParser(description="Finding a good solution to the autosharing problem.")
    parser.add_argument('input_file')
    parser.add_argument('output_file')
    parser.add_argument('time_limit_s', type=int)
    parser.add_argument('random_seed', type=int)
    parser.add_argument('thread_amount', type=int)
    argumentNamespace = parser.parse_args()

    #---------------Start of timing window---------------
    start_time = time.perf_counter()

    #Read input file and create model
    pi = ProcessInput(argumentNamespace.input_file)

    #Create initial solution
    reqsol = create_initial_input(pi.requests, pi.zones, pi.caramount)
    
    if((time.perf_counter() - start_time) < argumentNamespace.time_limit_s):
        #Find better solution
        exit

    elapsed_time = time.perf_counter() - start_time
    print(f"Elapsed time: {elapsed_time}s")
    #----------------End of timing window----------------

    ProcessOutput(argumentNamespace.output_file, reqsol)

