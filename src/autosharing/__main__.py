from __future__ import annotations
from typing import List
from autosharing.model import RequestStruct, Solution, ZoneStruct
from autosharing.input import ProcessInput
from autosharing.output import ProcessOutput
import sys
import time

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
    n = len(sys.argv)
    if n < 6 or n > 6:
        print("Not enough arguments, expecting: <input file> <output file> <time limit in s> <random seed> <number of threads>")
        sys.exit()
    print(f"Input file: {sys.argv[1]} | Output file: {sys.argv[2]} | Time limit(s): {sys.argv[3]} | Seed: {sys.argv[4]} | # of threads: {sys.argv[5]}")

    #---------------Start of timing window---------------
    start_time = time.perf_counter()

    #Read input file and create model
    pi = ProcessInput(sys.argv[1])

    #Create initial solution
    reqsol = create_initial_input(pi.requests, pi.zones, pi.caramount)
    
    if((time.perf_counter() - start_time) < int(sys.argv[3])):
        #Find better solution
        exit

    elapsed_time = time.perf_counter() - start_time
    print(f"Elapsed time: {elapsed_time}s")
    #----------------End of timing window----------------

    ProcessOutput(sys.argv[2], reqsol)

