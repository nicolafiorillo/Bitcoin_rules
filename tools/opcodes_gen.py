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

# Generate the Rust enum
rust_enum = "#[derive(Debug, Clone, Copy)]\n"
rust_enum += "pub enum OpCode {\n"
for opcode, hex_value in opcodes:
    rust_enum += f"    OP_{opcode} = 0x{hex_value},\n"
rust_enum += "}"

# Write the Rust enum to a file
filename = "../src/script/opcode.rs"
with open(filename, "w") as file:
    file.write(rust_enum)

print(f"Rust enum written to {filename}")
