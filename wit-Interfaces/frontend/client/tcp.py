import sys
import json
import asyncio
import ipaddress
import socket
from ipaddress import IPv4Address, IPv6Address
from command import exports
from typing import Tuple


class Run(exports.Run):
    def run(self):
        args = sys.argv[1:]
        if len(args) != 1:
            print(f"usage: tcp <address>:<port>", file=sys.stderr)
            exit(-1)

        address, port = parse_address_and_port(args[0])
        # Create a dictionary representing the JSON object
        payload1 = {"message_type": "test", "operation": "Insert", "name": "Bob"}
        data = asyncio.run(send_and_receive(address, port, json.dumps(payload1)))

        payload2 = {"message_type": "test", "operation": "Insert", "name": "Alice"}
        data = asyncio.run(send_and_receive(address, port, json.dumps(payload2)))

        payload3 = {"message_type": "test", "operation": "SELECT", "name": "Alice"}
        data = asyncio.run(send_and_receive(address, port, json.dumps(payload3)))

        print("Resonse received: ")
        print(data)


IPAddress = IPv4Address | IPv6Address


def parse_address_and_port(address_and_port: str) -> Tuple[IPAddress, int]:
    ip, separator, port = address_and_port.rpartition(":")
    assert separator
    return (ipaddress.ip_address(ip.strip("[]")), int(port))


async def send_and_receive(address: IPAddress, port: int, statement: str) -> str:
    data = ""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((address, port))
        s.sendall(statement.encode())

        buffSize = 1024

        data = s.recv(buffSize)
        print(f"received: {data}")

        s.close()
    return str(data)
