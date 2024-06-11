import sys
import asyncio
import ipaddress

from frontend_guest import frontend


class Run(frontend.Frontend):
    def run(self):
        args = sys.argv[1:]

        ip = args[0]
        port = int(args[1])

        client = Client()
        print("Starting client...")
        data = asyncio.run(client.get_sensor_data(ip, port))
        print(f"Received: {str(data)}")


class Client(frontend.Frontend):
    async def get_sensor_data(self, ip: str, port: int) -> str:
        address = ipaddress.ip_address(ip)
        rx, tx = await asyncio.open_connection(str(address), port)

        tx.write(b"Hello, world!")
        await tx.drain()

        data = await rx.read(1024)

        tx.close()
        await tx.wait_closed()

        return str(data)
