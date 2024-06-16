from .client import HostClient
from .parse import HostParse
from dataclasses import dataclass

@dataclass
class RootImports:
    parse: HostParse
    client: HostClient
