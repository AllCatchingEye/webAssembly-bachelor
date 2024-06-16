import webserver
# from webserver.imports import parse, client

# import json
# import random


class Webserver(webserver.Webserver):
    def start_webserver(self):
        return


# class Run(exports.Run):
#     def run(self):
#         return
# data = []
# for i in range(1, 11):
#     entry = {
#         "id": i,
#         "temperature": random.randint(
#             15, 30
#         ),  # Temperature between 15 and 30 degrees Celsius
#         "humidity": random.randint(30, 70),  # Humidity between 30% and 70%
#     }
#     data.append(entry)
#
# json_string = json.dumps(data, indent=2)
# print(json_string)
#
# ids = parse.parse_ids(json_string)
# temperatures = parse.parse_temperatures(json_string)
# humidities = parse.parse_humidities(json_string)
#
# payloads = [
#     {"message_type": "test", "operation": "Insert", "name": "Bob"},
#     {"message_type": "test", "operation": "Insert", "name": "Alice"},
#     {"message_type": "test", "operation": "Select", "name": "Alice"},
# ]
#
# ip = "127.0.0.1"
# port = 8080
# for payload in payloads:
#     data = client.get_sensor_data(ip, port, json.dumps(payload))
