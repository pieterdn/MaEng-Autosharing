
from .model import RequestStruct, ZoneStruct
import numpy as np
import numpy.typing as npt
import csv

class ProcessInput:
    def __init__(self) -> None:
        self.requests = []
        self.zones = []
        self.caramount = 0
        self.readCSV()

    def readCSV(self):
        with open('toy1.csv', newline='') as csv_file:
            reader = csv.reader(csv_file, delimiter=';')
            
            while 1:
                row = reader.__next__()
                if( "+" in row[0]):
                    splitted = row[0].split(": ")
                    match splitted[0]:
                        case "+Requests":
                            #print("Request case")
                            self.readRequests(reader, splitted[1])
                        case "+Zones":
                            #print("Zone case")
                            self.readZones(reader, splitted[1])
                        case "+Vehicles":
                            #print("Vehicles case")
                            self.caramount = int(splitted[1])
                        case "+Days":
                            #print("Days case")
                            break

    def readRequests(self, reader, amount):
        print("Processing requests...")
        for i in range(int(amount)):
            reqrow = reader.__next__()
            car_list = reqrow[5].split(",")
            for j in range(len(car_list)):
                car_list[j] = int((car_list[j].split("car"))[1])

            zoneid =    int((reqrow[1].split("z"))[1])
            day =       int(reqrow[2])
            start =     int(reqrow[3])
            time =      int(reqrow[4])
            cars =      np.array(car_list)
            pen1 =      int(reqrow[6])
            pen2 =      int(reqrow[7])

            newreq = RequestStruct(zoneid, day, start, time, cars, pen1, pen2)
            print(newreq)
            self.requests.append(newreq)

    def readZones(self, reader, amount):
        print("Processing zones...")
        for i in range(int(amount)):
            zonerow = reader.__next__()
            nextto_list = zonerow[1].split(",")
            zone_rel = np.zeros(int(amount), dtype=np.bool_)
            for j in range(len(nextto_list)):
                nextto_list[j] = int((nextto_list[j].split("z"))[1])
            for k in range(int(amount)):
                if(k in nextto_list):
                    zone_rel[k] = True
            newzone = ZoneStruct(zone_rel, np.array(nextto_list))
            print(newzone)
            self.zones.append(newzone)

if __name__ == "__main__":
    test = ProcessInput()
