from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class Run(Protocol):

    @abstractmethod
    def run(self) -> None:
        """
        Run the program.
        
        Raises: `frontend.types.Err(None)`
        """
        raise NotImplementedError


class Client(Protocol):

    @abstractmethod
    def get_sensor_data(self) -> str:
        raise NotImplementedError


