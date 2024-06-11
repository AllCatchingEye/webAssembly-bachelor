from ..intrinsics import _clamp, _decode_utf8, _encode_utf8, _load, _store
from ..types import Err, Ok, Result
import ctypes
from dataclasses import dataclass
from typing import List, Optional, Union
import wasmtime

from typing import TYPE_CHECKING
if TYPE_CHECKING:
  from .. import Root

@dataclass
class Bundled:
    module: str
    protocol: str
    name: str

@dataclass
class Function:
    protocol: str
    name: str

@dataclass
class Constructor:
    module: str
    protocol: str

@dataclass
class Static:
    module: str
    protocol: str
    name: str

@dataclass
class FunctionExportBundled:
    value: Bundled

@dataclass
class FunctionExportFreestanding:
    value: Function

@dataclass
class FunctionExportConstructor:
    value: Constructor

@dataclass
class FunctionExportMethod:
    value: str

@dataclass
class FunctionExportStatic:
    value: Static

FunctionExport = Union[FunctionExportBundled, FunctionExportFreestanding, FunctionExportConstructor, FunctionExportMethod, FunctionExportStatic]

@dataclass
class Case:
    name: str
    has_payload: bool

@dataclass
class LocalResource:
    new: int
    rep: int
    drop: int

@dataclass
class RemoteResource:
    drop: int

@dataclass
class Resource:
    local: Optional[LocalResource]
    remote: Optional[RemoteResource]

@dataclass
class OwnedKindRecord:
    value: List[str]

@dataclass
class OwnedKindVariant:
    value: List[Case]

@dataclass
class OwnedKindEnum:
    value: int

@dataclass
class OwnedKindFlags:
    value: int

@dataclass
class OwnedKindResource:
    value: Resource

OwnedKind = Union[OwnedKindRecord, OwnedKindVariant, OwnedKindEnum, OwnedKindFlags, OwnedKindResource]

@dataclass
class OwnedType:
    kind: OwnedKind
    package: str
    name: str

@dataclass
class TypeOwned:
    value: OwnedType

@dataclass
class TypeOption:
    pass

@dataclass
class TypeNestingOption:
    pass

@dataclass
class TypeResult:
    pass

@dataclass
class TypeTuple:
    value: int

@dataclass
class TypeHandle:
    pass

Type = Union[TypeOwned, TypeOption, TypeNestingOption, TypeResult, TypeTuple, TypeHandle]

@dataclass
class Symbols:
    types_package: str
    exports: List[FunctionExport]
    types: List[Type]

class Exports:
    component: 'Root'
    
    def __init__(self, component: 'Root') -> None:
        self.component = component
    def init(self, caller: wasmtime.Store, app_name: str, symbols: Symbols, stub_wasi: bool) -> Result[None, str]:
        ptr, len0 = _encode_utf8(app_name, self.component._realloc0, self.component._core_memory0, caller)
        record = symbols
        field = record.types_package
        field1 = record.exports
        field2 = record.types
        ptr3, len4 = _encode_utf8(field, self.component._realloc0, self.component._core_memory0, caller)
        vec = field1
        len46 = len(vec)
        result = self.component._realloc0(caller, 0, 0, 4, len46 * 28)
        assert(isinstance(result, int))
        for i47 in range(0, len46):
            e = vec[i47]
            base5 = result + i47 * 28
            if isinstance(e, FunctionExportBundled):
                payload = e.value
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base5, 0, 0)
                record6 = payload
                field7 = record6.module
                field8 = record6.protocol
                field9 = record6.name
                ptr10, len11 = _encode_utf8(field7, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 8, len11)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 4, ptr10)
                ptr12, len13 = _encode_utf8(field8, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 16, len13)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 12, ptr12)
                ptr14, len15 = _encode_utf8(field9, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 24, len15)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 20, ptr14)
            elif isinstance(e, FunctionExportFreestanding):
                payload16 = e.value
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base5, 0, 1)
                record17 = payload16
                field18 = record17.protocol
                field19 = record17.name
                ptr20, len21 = _encode_utf8(field18, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 8, len21)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 4, ptr20)
                ptr22, len23 = _encode_utf8(field19, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 16, len23)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 12, ptr22)
            elif isinstance(e, FunctionExportConstructor):
                payload24 = e.value
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base5, 0, 2)
                record25 = payload24
                field26 = record25.module
                field27 = record25.protocol
                ptr28, len29 = _encode_utf8(field26, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 8, len29)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 4, ptr28)
                ptr30, len31 = _encode_utf8(field27, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 16, len31)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 12, ptr30)
            elif isinstance(e, FunctionExportMethod):
                payload32 = e.value
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base5, 0, 3)
                ptr33, len34 = _encode_utf8(payload32, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 8, len34)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 4, ptr33)
            elif isinstance(e, FunctionExportStatic):
                payload35 = e.value
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base5, 0, 4)
                record36 = payload35
                field37 = record36.module
                field38 = record36.protocol
                field39 = record36.name
                ptr40, len41 = _encode_utf8(field37, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 8, len41)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 4, ptr40)
                ptr42, len43 = _encode_utf8(field38, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 16, len43)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 12, ptr42)
                ptr44, len45 = _encode_utf8(field39, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 24, len45)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base5, 20, ptr44)
            else:
                raise TypeError("invalid variant specified for FunctionExport")
        vec101 = field2
        len103 = len(vec101)
        result102 = self.component._realloc0(caller, 0, 0, 4, len103 * 48)
        assert(isinstance(result102, int))
        for i104 in range(0, len103):
            e48 = vec101[i104]
            base49 = result102 + i104 * 48
            if isinstance(e48, TypeOwned):
                payload50 = e48.value
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 0, 0)
                record51 = payload50
                field52 = record51.kind
                field53 = record51.package
                field54 = record51.name
                if isinstance(field52, OwnedKindRecord):
                    payload55 = field52.value
                    _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 4, 0)
                    vec60 = payload55
                    len62 = len(vec60)
                    result61 = self.component._realloc0(caller, 0, 0, 4, len62 * 8)
                    assert(isinstance(result61, int))
                    for i63 in range(0, len62):
                        e56 = vec60[i63]
                        base57 = result61 + i63 * 8
                        ptr58, len59 = _encode_utf8(e56, self.component._realloc0, self.component._core_memory0, caller)
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base57, 4, len59)
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base57, 0, ptr58)
                    _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 12, len62)
                    _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 8, result61)
                elif isinstance(field52, OwnedKindVariant):
                    payload64 = field52.value
                    _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 4, 1)
                    vec72 = payload64
                    len74 = len(vec72)
                    result73 = self.component._realloc0(caller, 0, 0, 4, len74 * 12)
                    assert(isinstance(result73, int))
                    for i75 in range(0, len74):
                        e65 = vec72[i75]
                        base66 = result73 + i75 * 12
                        record67 = e65
                        field68 = record67.name
                        field69 = record67.has_payload
                        ptr70, len71 = _encode_utf8(field68, self.component._realloc0, self.component._core_memory0, caller)
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base66, 4, len71)
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base66, 0, ptr70)
                        _store(ctypes.c_uint8, self.component._core_memory0, caller, base66, 8, int(field69))
                    _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 12, len74)
                    _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 8, result73)
                elif isinstance(field52, OwnedKindEnum):
                    payload76 = field52.value
                    _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 4, 2)
                    _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 8, _clamp(payload76, 0, 4294967295))
                elif isinstance(field52, OwnedKindFlags):
                    payload77 = field52.value
                    _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 4, 3)
                    _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 8, _clamp(payload77, 0, 4294967295))
                elif isinstance(field52, OwnedKindResource):
                    payload78 = field52.value
                    _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 4, 4)
                    record79 = payload78
                    field80 = record79.local
                    field81 = record79.remote
                    if field80 is None:
                        _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 8, 0)
                    else:
                        payload83 = field80
                        _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 8, 1)
                        record84 = payload83
                        field85 = record84.new
                        field86 = record84.rep
                        field87 = record84.drop
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 12, _clamp(field85, 0, 4294967295))
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 16, _clamp(field86, 0, 4294967295))
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 20, _clamp(field87, 0, 4294967295))
                    if field81 is None:
                        _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 24, 0)
                    else:
                        payload89 = field81
                        _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 24, 1)
                        record90 = payload89
                        field91 = record90.drop
                        _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 28, _clamp(field91, 0, 4294967295))
                else:
                    raise TypeError("invalid variant specified for OwnedKind")
                ptr92, len93 = _encode_utf8(field53, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 36, len93)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 32, ptr92)
                ptr94, len95 = _encode_utf8(field54, self.component._realloc0, self.component._core_memory0, caller)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 44, len95)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 40, ptr94)
            elif isinstance(e48, TypeOption):
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 0, 1)
            elif isinstance(e48, TypeNestingOption):
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 0, 2)
            elif isinstance(e48, TypeResult):
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 0, 3)
            elif isinstance(e48, TypeTuple):
                payload99 = e48.value
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 0, 4)
                _store(ctypes.c_uint32, self.component._core_memory0, caller, base49, 4, _clamp(payload99, 0, 4294967295))
            elif isinstance(e48, TypeHandle):
                _store(ctypes.c_uint8, self.component._core_memory0, caller, base49, 0, 5)
            else:
                raise TypeError("invalid variant specified for Type")
        ret = self.component.lift_callee0(caller, ptr, len0, ptr3, len4, result, len46, result102, len103, int(stub_wasi))
        assert(isinstance(ret, int))
        load = _load(ctypes.c_uint8, self.component._core_memory0, caller, ret, 0)
        expected: Result[None, str]
        if load == 0:
            expected = Ok(None)
        elif load == 1:
            load105 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 4)
            load106 = _load(ctypes.c_int32, self.component._core_memory0, caller, ret, 8)
            ptr107 = load105
            len108 = load106
            list = _decode_utf8(self.component._core_memory0, caller, ptr107, len108)
            expected = Err(list)
        else:
            raise TypeError("invalid variant discriminant for expected")
        self.component._post_return0(caller, ret)
        return expected
    