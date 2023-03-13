from __future__ import annotations
from typing import List
from .model import RequestStruct, Solution, ZoneStruct


def create_initial_input(reqs: List[RequestStruct],
                         zones: List[ZoneStruct],
                         amount_cars: int) -> RequestSolution:
    reqsol = RequestSolution(len(reqs), amount_cars)
    for req in reqs:
        pass


if __name__ == "__main__":
    # input stuff
    requestInfo: List[RequestStruct] = list()
