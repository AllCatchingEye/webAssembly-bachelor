from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Client(Protocol):

    @abstractmethod
    def get_sensor_data(self, ip: str, port: int, payload: str) -> str:
        raise NotImplementedError


