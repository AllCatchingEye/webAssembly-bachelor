from abc import abstractmethod
from typing import Protocol

class HostClient(Protocol):
    @abstractmethod
    def get_sensor_data(self, ip: str, port: int, payload: str) -> str:
        raise NotImplementedError

