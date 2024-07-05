import socket

import tcp_client


class Client(tcp_client.TcpClient):
    def get_sensor_data(self, ip: str, port: int, payload: str) -> str:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((ip, port))
            s.sendall(payload.encode())

            buffSize = 1024
            data = s.recv(buffSize).decode()

        return data
