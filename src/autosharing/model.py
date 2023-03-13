from __future__ import annotations
import numpy as np
from typing import NamedTuple, List, Set
import numpy.typing as npt

MINUTES_IN_DAY = 1440

class Solution:
    req_to_car: npt.NDArray[np.int16]
    car_to_req: npt.NDArray[np.bool_]
    car_to_reqNumber: List[Set[int]]
    zone_to_car: npt.NDArray[np.bool_]
    car_to_zone: npt.NDArray[np.int16]
    reqs: List[RequestStruct]
    zones: List[ZoneStruct]
    cost_per_req: npt.NDArray[np.int16]
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
        self.car_to_req = np.zeros((num_reqs, num_cars), dtype=np.bool_)
        self.zone_to_car = np.zeros((len(zones), num_cars), dtype=np.bool_)
        self.car_to_zone = np.full(num_cars, -1, dtype=np.int16)
        self.reqs = reqs
        self.zones = zones
        self.car_to_reqNumber = [set()]*num_cars

    def feasibleCarToReq(self, req: int, car: int) -> bool:
        # zelfde req zelfde auto?
        if self.car_to_zone[car] < 0:
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

    def costOfCar(self, req: int, car: int) -> int:
        zone_car = self.car_to_zone[car]
        req_struct = self.reqs[req]
        if self.zones[zone_car].nextto[req_struct.zone]:
            return req_struct.pen2
        elif zone_car == req_struct.zone:
            return 0
        return req_struct.pen1

    def addCarToReq(self, req: int, car: int):
        old_cost = self.cost_per_req[req]
        new_cost = self.costOfCar(req, car)
        self.req_to_car[req] = car
        self.car_to_req[car][req] = 1
        self.car_to_reqNumber[car].add(req)
        self.cost += new_cost - old_cost



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
