#!/usr/bin/env python3

# get a string from parameter and prinf the hex representation

import sys

if len(sys.argv) < 2:
    print("No command passed")
    exit()

command = sys.argv[1]

command.encode('utf-8')
res = ""

for c in command:
    res += f"0x{c.encode('utf-8').hex().upper()}, "

for i in range(0, 12 - len(command)):
    res += "0x00, "

res = res.rstrip(", ")

print(f"'{command}' = [{res}]")
