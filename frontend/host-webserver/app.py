from guest import test

import random
import json
import socket


class Test(test.Test):
    def run(self) -> None:
        data = []
        for i in range(1, 11):
            entry = {
                "id": i,
                "temperature": random.randint(
                    15, 30
                ),  # Temperature between 15 and 30 degrees Celsius
                "humidity": random.randint(30, 70),  # Humidity between 30% and 70%
            }
            data.append(entry)

        json_string = json.dumps(data, indent=2)
        print(json_string)

        ids = self.parse_ids(json_string)
        temperatures = self.parse_temperatures(json_string)
        humidities = self.parse_humidities(json_string)

        print(ids)
        print(temperatures)
        print(humidities)

        payloads = [
            {"message_type": "test", "operation": "Insert", "name": "Bob"},
            {"message_type": "test", "operation": "Insert", "name": "Alice"},
            {"message_type": "test", "operation": "Select", "name": "Alice"},
        ]

        ip = "127.0.0.1"
        port = 8080
        for payload in payloads:
            data = self.get_sensor_data(ip, port, json.dumps(payload))
        return

    def get_sensor_data(self, ip: str, port: int, payload: str) -> str:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((ip, port))
            s.sendall(payload.encode())

            buffSize = 1024
            data = s.recv(buffSize).decode()

        return data

    def parse_ids(self, json_str: str) -> list[int]:
        print("Parser is called")
        data = json.loads(json_str)

        ids = [int(item["id"]) for item in data]

        print("Returning ids")
        print(ids)

        return ids

    def parse_temperatures(self, json_str: str) -> list[int]:
        print("Parser is called")
        data = json.loads(json_str)

        temperatures = [item["temperature"] for item in data]

        print("Returning temperatures")
        print(temperatures)

        return temperatures

    def parse_humidities(self, json_str: str) -> list[int]:
        print("Parser is called")
        data = json.loads(json_str)

        humidities = [item["humidity"] for item in data]

        print("Returning humidities")
        print(humidities)

        return humidities
