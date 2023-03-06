
import numpy as np
from typing import NamedTuple

class RequestStruct(NamedTuple):
    zone: int
    day: int
    start: int
    time: int
    cars: np.array
    pen1: int
    pen2: int

class ZoneStruct(NamedTuple):
    zonerel: np.array
    nextto: np.array