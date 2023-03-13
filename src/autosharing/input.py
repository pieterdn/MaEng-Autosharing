
from model import RequestStruct, ZoneStruct
import numpy as np
import numpy.typing as npt
import csv

class ProcessInput:
    def __init__(self) -> None:
        self.requests = []
        self.zones = []

    def readCSV(self):
        with open('toy1.csv', newline='') as csv_file:
            reader = csv.reader(csv_file, delimiter=',')
            #for row in reader:
                #print(row)
            #if( "+" in row[0]):
            
            while 1:
                row = reader.__next__()
                if( "+" in row[0]):
                    splitted = row[0].split(": ")
                    match splitted[0]:
                        case "+Requests":
                            print("Request case")
                            self.readRequests(reader, splitted[1])
                        case "+Zones":
                            print("Zone case")
                            self.readZones(reader, splitted[1])
                        case "+Vehicles":
                            print("Vehicles case")
                        case "+Days":
                            print("Days case")

    def readRequests(self, reader, amount):
        print("Processing requests...")
        for i in range(int(amount)):
            reqrow = reader.__next__()
            #splitted = reqrow.split(";")
            print(reqrow)

    def readZones(self, reader, amount):
        print("Processing zones...")

test = ProcessInput()
test.readCSV()