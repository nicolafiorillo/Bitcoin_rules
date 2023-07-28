#!/usr/bin/env python3

import requests
import re

# Fetch the script.h file from GitHub
url = "https://raw.githubusercontent.com/bitcoin/bitcoin/600c595b8d2f4bf049b9182d4a0aa88e4b34458d/src/script/script.h"
response = requests.get(url)

file_contents = response.text

# Extract opcode names and hex values using regular expressions
opcode_pattern = r"\s*OP_(\w+)\s+=\s+0x([0-9a-fA-F]+),"
opcodes = re.findall(opcode_pattern, file_contents)

rust_enum = ""
for opcode, hex_value in opcodes:
    rust_enum += f"pub const OP_{opcode}: OpCode = 0x{hex_value.upper()};\n"

rust_enum += ""

for opcode, hex_value in opcodes:
    rust_enum += f"op_to_fn[OP_{opcode}] = not_implemented;\n"

filename = "../src/scripting/opcode.txt"
with open(filename, "w") as file:
    file.write(rust_enum)

print(f"Op codes written to {filename}")
