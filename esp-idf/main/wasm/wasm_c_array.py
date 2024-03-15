import sys


class WasmData:
    def __init__(self, data):
        self.data = data

    @classmethod
    def from_bytes(cls, arr):
        return cls(arr)

    @classmethod
    def from_file(cls, filename):
        with open(filename, "rb") as f:
            return cls.from_bytes(f.read())

    def to_c_array(self, array_name="bm_wasm_prog", size_name="bm_wasm_prog_s"):
        out = f"unsigned char __aligned(4) {array_name}[] = {{"

        data_len = len(self.data)

        for i, b in enumerate(self.data):
            if i % 23 == 0:
                out += "\n    "

            out += "0x" + f"{b:x}".rjust(2, "0")

            if i < data_len - 1:
                out += ","

        out += "\n};\n"

        return out


filename = sys.argv[1]
program = sys.argv[2]

data = WasmData.from_file(filename)
out = data.to_c_array(array_name=program)

f = open(f"../c_arrays/{program}.h", "w")
f.write(out)
f.close()
