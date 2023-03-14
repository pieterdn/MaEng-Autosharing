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
    reqsol = Solution(len(reqs), amount_cars, reqs, zones)
    for i, req in enumerate(reqs):
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
    #----------------End of timing window----------------

    print(f"Elapsed time: {elapsed_time}s")
    ProcessOutput(argumentNamespace.output_file, reqsol)

