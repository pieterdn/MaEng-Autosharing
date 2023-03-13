from __future__ import annotations
from typing import List
from autosharing.model import RequestStruct, Solution, ZoneStruct
from autosharing.input import ProcessInput
from autosharing.output import ProcessOutput


def create_initial_input(reqs: List[RequestStruct],
                         zones: List[ZoneStruct],
                         amount_cars: int) -> Solution:
    print(len(reqs), amount_cars)
    reqsol = Solution(len(reqs), amount_cars, reqs, zones)
    for i, req in enumerate(reqs):
        print(reqsol.req_to_car)
        print(reqsol.car_to_zone)
        for car in req.cars:
            print(f"car: {car} req: {req}")
            zone = reqsol.car_to_zone[car]
            print(f"zone{zone}")
            if zone < 0:
                print("here1")
                new_zone = req.zone
                reqsol.changeCost(i, 0)
                print(f"car: {car} zone: {new_zone}")
                reqsol.zoneHardChange(car, new_zone)
                reqsol.carHardChange(i, car)
                break
            else:
                print("here2")
                if not reqsol.feasibleCarToReq(i, car):
                    print(f"not feasible: req:{i} car:{car}")
                    continue
                new_cost, pen = reqsol.costOfZone(i, zone)
                print(f"req: {i}, pen: {pen}")
                if pen == 0 or pen == 2:
                    reqsol.changeCost(i, new_cost)
                    reqsol.carHardChange(i, car)
                    break
    for i, zone in enumerate(reqsol.car_to_zone):
        if zone < 0:
            reqsol.car_to_zone[i] = 0
    return reqsol



if __name__ == "__main__":
    pi = ProcessInput()
    reqsol = create_initial_input(pi.requests, pi.zones, pi.caramount)
    print(reqsol.req_to_car)
    print(reqsol.car_to_zone)
    ProcessOutput(reqsol)

