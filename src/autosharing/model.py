from __future__ import annotations
import numpy as np
from typing import NamedTuple, List, Set, Tuple
import numpy.typing as npt

MINUTES_IN_DAY = 1440

class Solution:
    req_to_car: npt.NDArray[np.int16]           # Give req as index get car
    req_to_car_bools: npt.NDArray[np.bool_]     # 2D bool array: a row with len = #cars for every request
    car_to_reqNumber: List[Set[int]]            # Give car as index get list of reqs
    car_to_zone_bools: npt.NDArray[np.bool_]    # 2D bool array: a row with len = #cars for every zone
    car_to_zone: npt.NDArray[np.int16]          # Give car as index get zone
    reqs: List[RequestStruct]                   # List of requests
    zones: List[ZoneStruct]                     # List of zones
    cost_per_req: npt.NDArray[np.int16]         # Give req as index get cost
    cost: int

    def __init__(self, num_reqs: int,
                 num_cars: int,
                 reqs: List[RequestStruct],
                 zones: List[ZoneStruct]) -> None:
        self.req_to_car = np.full(num_reqs, -1, dtype=np.int16)
        self.cost_per_req = np.zeros(len(reqs), dtype=np.int16)
        self.cost = 0
        for i, req in enumerate(reqs):
            self.cost_per_req[i] = req.pen1
            self.cost += req.pen1
        self.req_to_car_bools = np.zeros((num_reqs, num_cars), dtype=np.bool_)
        self.car_to_zone_bools = np.zeros((len(zones), num_cars), dtype=np.bool_)
        self.car_to_zone = np.full(num_cars, -1, dtype=np.int16)
        self.reqs = reqs
        self.zones = zones
        self.car_to_reqNumber = [set() for _ in range(num_cars)]

    def feasibleCarToReq(self, req: int, car: int) -> bool:
        # zelfde req zelfde auto?
        zone = self.car_to_zone[car] 
        if zone < 0:
            return False
        _, feasible = self.costAndFeasibleZone(req, zone)
        if not feasible:
            return False
        req_struct = self.reqs[req]
        req_start = req_struct.day*MINUTES_IN_DAY + req_struct.start
        req_end = req_start + req_struct.time
        for alloc_req in self.car_to_reqNumber[car]:
            areq_struct = self.reqs[alloc_req]
            areq_start = areq_struct.day*MINUTES_IN_DAY + areq_struct.start
            areq_end = areq_start + areq_struct.time
            if len(range(max(req_start, areq_start), min(req_end, areq_end)+1)) != 0:
                return False
        return True

    def costAndFeasibleZone(self, req: int, zone: int) -> Tuple[int, bool]:
        req_struct = self.reqs[req]
        if self.zones[zone].zonerel[req_struct.zone]:
            return (req_struct.pen2, True)
        elif zone == req_struct.zone:
            return (0, True)
        return (req_struct.pen1, False)

    def costOfCar(self, req: int, car: int) -> Tuple[int, bool]:
        zone_car = self.car_to_zone[car]
        return self.costAndFeasibleZone(req, zone_car)

    def changeCarZone(self, car: int, zone: int):
        for req in self.car_to_reqNumber[car]:
            # req_struct = self.reqs[req]
            new_cost, feasible = self.costAndFeasibleZone(req, zone)
            if feasible:
                self.changeCost(req, new_cost)
            else:
                self.changeCost(req, new_cost)
                self.carHardChange(req, -1)
        self.zoneHardChange(car, zone)

    def changeCost(self, req: int, new_cost: int):
        old_cost = self.cost_per_req[req]
        self.cost_per_req[req] = new_cost
        self.cost += new_cost - old_cost

    def addCarToReq(self, req: int, car: int):
        new_cost = self.costOfCar(req, car)[0]
        self.carHardChange(req, car)
        self.changeCost(req, new_cost)

    def zoneHardChange(self, car: int, zone: int):
        old_zone = self.car_to_zone[car]
        self.car_to_zone_bools[old_zone][car] = False
        if zone >= 0:
            self.car_to_zone[car] = zone
            self.car_to_zone_bools[zone][car] = True

    def carHardChange(self, req: int, car: int):
        old_car = self.req_to_car[req]
        self.req_to_car[req] = car
        if old_car >= 0:
            self.req_to_car_bools[req][old_car] = False
            self.car_to_reqNumber[old_car].remove(req)
        if car >= 0:
            self.req_to_car_bools[req][car] = True
            self.car_to_reqNumber[car].add(req)


class RequestStruct(NamedTuple):
    '''
    Elke request (reservatie) is van het type RequestStruct
    '''
    zone:   int                     # ZoneID waar reservatie gemaakt werd
    day:    int                     # Dag waarop reservatie start (gedefinieerd als index)
    start:  int                     # Starttijd van reservatie (minuten vanaf middernacht)
    time:   int                     # Duurtijd van de reservatie
    cars:   npt.NDArray[np.int16]   # Voertuiglijst waaraan deze reservatie kan worden toegewezen
    pen1:   int                     # Kost om reservatie niet toe te wijzen aan voertuig
    pen2:   int                     # Kost om reservatie toe te wijzen in aanliggende zone

class ZoneStruct(NamedTuple):
    '''
    Elke zone is van het type ZoneStruct
    '''
    zonerel:    npt.NDArray[np.bool_]   # Relatie van een zone tov alle andere 1=aanl, 0 niet
    nextto:     npt.NDArray[np.int16]   # Lijst die aanliggende zones aangeeft voor een zone
