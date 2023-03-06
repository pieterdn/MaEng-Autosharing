from __future__ import annotations
import numpy as np
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
