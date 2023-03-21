from __future__ import annotations
import numpy as np
from typing import NamedTuple, List, Set, Tuple
import numpy.typing as npt
import copy
from dataclasses import dataclass

MINUTES_IN_DAY = 1440
NUM_OF_PEN = 3


@dataclass
class SolutionModel:
    req_to_car: npt.NDArray[np.int16]           # Give req as index get car
    car_to_zone: npt.NDArray[np.int16]          # Give car as index get zone
    reqs: List[RequestStruct]                   # List of requests
    zones: List[ZoneStruct]                     # List of zones
    cost: int


class TransactionReq(NamedTuple):
    req: int
    carFrom: int
    carTo: int
    costFrom: int
    costTo: int

class TransactionCar(NamedTuple):
    car: int
    zoneFrom: int
    zoneTo: int

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
    in_trans: bool
    transaction: List[TransactionReq | TransactionCar]

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
        self.req_to_pen = np.full((NUM_OF_PEN, num_reqs), 1, dtype=np.int16)
        self.reqs = reqs
        self.zones = zones
        self.car_to_reqNumber = [set() for _ in range(num_cars)]
        self.in_trans = False
        self.transaction = []

    def startTransaction(self):
        self.in_trans = True

    def commit(self):
        self.in_trans = False
        self.transaction = []

    def rollback(self):
        self.in_trans = False
        for trans in reversed(self.transaction):
            if isinstance(trans, TransactionReq):
                self.req_to_car[trans.req] = trans.carFrom
                if trans.carTo >= 0:
                    self.req_to_car_bools[trans.req][trans.carTo] = False
                    self.car_to_reqNumber[trans.carTo].remove(trans.req)
                if trans.carFrom >= 0:
                    self.req_to_car_bools[trans.req][trans.carFrom] = True
                    self.car_to_reqNumber[trans.carFrom].add(trans.req)
                self.cost_per_req[trans.req] = trans.costFrom
                self.cost += trans.costFrom - trans.costTo
            else:
                self.car_to_zone[trans.car] = trans.zoneFrom
                self.car_to_zone_bools[trans.zoneTo][trans.car] = False
                self.car_to_zone_bools[trans.zoneFrom][trans.car] = True
        self.transaction = []

    def toModel(self) -> SolutionModel:
        return SolutionModel(
            np.copy(self.req_to_car),
            np.copy(self.car_to_zone),
            self.reqs,
            self.zones,
            self.cost
        )

    """
    Takes reqid and carid and checks whether the car could be feasible for the 
    request.
    """
    def feasibleCarToReq(self, req: int, car: int) -> bool:
        # zelfde req zelfde auto?
        zone = self.car_to_zone[car] 
        if zone < 0:
            return False
        req_struct = self.reqs[req]
        ok = False
        for cr in req_struct.cars:
            if cr == car:
                ok = True
        if not ok:
            return False
        _, feasible = self.costAndFeasibleZone(req, zone)
        if not feasible:
            return False
        req_start = req_struct.day*MINUTES_IN_DAY + req_struct.start
        req_end = req_start + req_struct.time
        for alloc_req in self.car_to_reqNumber[car]:
            areq_struct = self.reqs[alloc_req]
            areq_start = areq_struct.day*MINUTES_IN_DAY + areq_struct.start
            areq_end = areq_start + areq_struct.time
            if len(range(max(req_start, areq_start), min(req_end, areq_end)+1)) != 0:
                return False
        return True

    """
    Takes a reqid and the zone of a car, checks whether the zone of the car is 
    feasible in combination with the req and returns corresponding cost and bool.
    """
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

    def changeCarZone(self, car: int, zone: int) -> List[int]:
        lst: List[int] = list()
        reqs = copy.copy(self.car_to_reqNumber[car])
        for req in reqs:
            # req_struct = self.reqs[req]
            new_cost, feasible = self.costAndFeasibleZone(req, zone)
            if feasible:
                if self.in_trans:
                    self.transaction.append(
                        TransactionReq(
                            req,
                            car,
                            car,
                            self.cost_per_req[req],
                            new_cost
                        )
                    )
                self.changeCost(req, new_cost)
            else:
                if self.in_trans:
                    self.transaction.append(
                        TransactionReq(
                            req,
                            car,
                            -1,
                            self.cost_per_req[req],
                            new_cost
                        )
                    )
                lst.append(req)
                self.changeCost(req, new_cost)
                self.carHardChange(req, -1)
        if self.in_trans:
            self.transaction.append(
                TransactionCar(
                    car,
                    self.car_to_zone[car],
                    zone
                )
            )
        self.zoneHardChange(car, zone)
        return lst

    """
    Takes a reqid and a new cost and updates the cost_per_req and cost values.
    """
    def changeCost(self, req: int, new_cost: int):
        old_cost = self.cost_per_req[req]
        self.cost_per_req[req] = new_cost
        self.cost += new_cost - old_cost

    def newCost(self, req: int, car: int) -> int:
        new_cost, _ = self.costOfCar(req, car)
        old_cost = self.cost_per_req[req]
        return (self.cost + new_cost - old_cost)

    def addCarToReq(self, req: int, car: int):
        new_cost = self.costOfCar(req, car)[0]
        if self.in_trans:
            self.transaction.append(
                TransactionReq(
                    req,
                    self.req_to_car[req],
                    car,
                    self.cost_per_req[req],
                    new_cost
                )
            )
        self.carHardChange(req, car)
        self.changeCost(req, new_cost)

    """
    Takes a carid and a zone and updates car_to_zone and car_to_zone_bools.
    """
    def zoneHardChange(self, car: int, zone: int):
        old_zone = self.car_to_zone[car]
        self.car_to_zone_bools[old_zone][car] = False
        if zone >= 0:
            self.car_to_zone[car] = zone
            self.car_to_zone_bools[zone][car] = True

    """
    Takes a reqid and a carid and updates req_to_car, req_to_car_bools and car_to_reqNumber.
    """
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
