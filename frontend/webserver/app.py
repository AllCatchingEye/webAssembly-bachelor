import webserver
from webserver import print_host
from webserver.imports import parse, client, plot

import time
import json
import random


def test_parse_plot():
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
    print_host(json_string)

    ids = parse.parse_ids(json_string)
    temperatures = parse.parse_temperatures(json_string)
    humidities = parse.parse_humidities(json_string)

    print_host("Parsed ids:")
    print_host(str(ids))
    print_host("Parsed temperatures:")
    print_host(str(temperatures))
    print_host("Parsed humidities:")
    print_host(str(humidities))

    plot.plot_temperature(ids, temperatures)
    plot.plot_humidity(ids, humidities)


class Webserver(webserver.Webserver):
    def start_webserver(self):
        payloads = (
            {
                "message_type": "dht11",
                "operation": "Insert",
                "temperature": 26,
                "humidity": 62,
            },
            {
                "message_type": "dht11",
                "operation": "Insert",
                "temperature": 27,
                "humidity": 64,
            },
            {
                "message_type": "dht11",
                "operation": "Insert",
                "temperature": 28,
                "humidity": 66,
            },
        )

        ip = "192.168.0.217"
        port = 8080

        print_host("Inserting sensor data...")
        for payload in payloads:
            data = client.get_sensor_data(ip, port, json.dumps(payload))

        print_host("Starting main loop...")
        while True:
            payload = {"message_type": "dht11", "operation": "Select"}

            try:
                print_host("Requesting data...")
                data = client.get_sensor_data(ip, port, json.dumps(payload))

                json_string = json.dumps(data, indent=2)

                ids = parse.parse_ids(json_string)
                temperatures = parse.parse_temperatures(json_string)
                humidities = parse.parse_humidities(json_string)

                plot.plot_temperature(ids, temperatures)
                plot.plot_humidity(ids, humidities)
            except:
                print_host("Coudn't connect to backend server")
            finally:
                time.sleep(10)
