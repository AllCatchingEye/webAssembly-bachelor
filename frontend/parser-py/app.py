import parser

import json


class Parse(parser.Parser):
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
