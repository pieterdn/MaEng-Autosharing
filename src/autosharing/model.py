from __future__ import annotations
import numpy as np
from typing import NamedTuple
import numpy.typing as npt


class RequestSolution:
    req_to_car: npt.NDArray[np.int16]

    def __init__(self, num_reqs, num_cars) -> None:
        self.req_to_car = np.full(num_reqs, -1, dtype=np.int16)

    def __getitem__(self, index: int) -> int | None:
        car = self.req_to_car[index]
        if car < 0:
            return None
        return car

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
    zonerel:    npt.NDArray[np.bool8]   # Relatie van een zone tov alle andere 1=aanl, 0 niet
    nextto:     npt.NDArray[np.int16]   # Lijst die aanliggende zones aangeeft voor een zone
