from __future__ import annotations
from typing import List
from .model import RequestStruct, Solution, ZoneStruct


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
                break
            else:
                new_cost, pen = reqsol.costOfZone(i, zone)
                if pen == 0 or 2:
                    reqsol.changeCost(i, new_cost)
                    reqsol.carHardChange(i, car)
                    break
    return reqsol



if __name__ == "__main__":
    # input stuff
    requestInfo: List[RequestStruct] = list()
