from abc import abstractmethod
from typing import List, Protocol

class HostParse(Protocol):
    @abstractmethod
    def parse_ids(self, json: str) -> List[int]:
        raise NotImplementedError
    @abstractmethod
    def parse_temperatures(self, json: str) -> List[int]:
        raise NotImplementedError
    @abstractmethod
    def parse_humidities(self, json: str) -> List[int]:
        raise NotImplementedError

