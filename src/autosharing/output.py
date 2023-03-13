
from model import Solution
import csv

class ProcessOutput:
    def __init__(self, solution: Solution) -> None:
        self.car_to_zone =  solution.car_to_zone
        self.req_to_car = solution.req_to_car
        self.cost = solution.cost
        self.writeCSV()

    def writeCSV(self):
        with open('solution.csv', 'w', newline='') as csv_file:
            writer = csv.writer(csv_file)
            writer.writerow(self.cost)
            writer.writerow("+Vehicle assignments")
            for i in range(len(self.car_to_zone)):
                car = "car" + str(i)
                zone = "z" + str(self.car_to_zone[i])
                writer.writerow(car + ';' + zone)

            writer.writerow("+Assigned requests")
            for j in range(len(self.req_to_car)):
                if(self.req_to_car[j] != -1):
                    req = "req" + str(j)
                    car = "car" + str(self.req_to_car[j])
                    writer.writerow(req + ';' + car)

            writer.writerow("+Unassigned requests")
            for k in range(len(self.req_to_car)):
                if(self.req_to_car[k] == -1):
                    req = "req" + str(k)
                    writer.writerow(req)
