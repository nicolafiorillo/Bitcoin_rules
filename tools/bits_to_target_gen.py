#!/usr/bin/env python3

# Create test fixture for bits_to_target and target_to_bits functions

MAX_BITS = 0x1D00FFFF
MAX_TARGET = 0x00000000FFFF0000000000000000000000000000000000000000000000000000

def little_endian_to_int(b):
    return int.from_bytes(b, 'little')

def bits_to_target(bits):
    exponent = bits[-1]
    coefficient = little_endian_to_int(bits[:-1])
    return coefficient * 256**(exponent - 3)


def target_to_bits(target):
    raw_bytes = target.to_bytes(32, 'big')
    raw_bytes = raw_bytes.lstrip(b'\x00')

    if raw_bytes[0] > 0x7f:
        exponent = len(raw_bytes) + 1
        coefficient = b'\x00' + raw_bytes[:2]
    else:
        exponent = len(raw_bytes)
        coefficient = raw_bytes[:3]

    new_bits = coefficient[::-1] + bytes([exponent])
    return new_bits

errors = 0
samples = 0
over = 0

values = [0x0000FFFF, 0x0000FEFD, 0x0000EEEE, 0x0000DDDD, 0x0000CCCC, 0x0000BBBB, 0x0000AAAA, 0x00009999, 0x00008888, 0x00008000, 0x007FFFFF]

for n in values:
    for i in range(3, 30):
        b = n + (256 ** 3) * i
        if b > MAX_BITS:
            over += 1
            print(f"{i:03}: Bits {b} > MAX_BITS {MAX_BITS}")
            continue
    
        b_array = b.to_bytes(4, 'little')

        target = bits_to_target(b_array)

        padded_target = ("%0.2X" % target).zfill(64)

        bits_verify = target_to_bits(target)

        err = ""
        if b_array != bits_verify:
            print(f"b_array: {b_array} - bits_verify: {bits_verify}")
            err = "ERROR"
            errors += 1

        # print(f"{i:03}: 0x{padded_target} - 0x{bits_verify[::-1].hex().upper()} {err}")
        # print(f"target_and_bits!(t2b_{samples}, b2t_{samples}, \"{padded_target}\", 0x{bits_verify[::-1].hex().upper()});")
        print(f"(\"{padded_target}\", 0x{bits_verify[::-1].hex().upper()}),")
        samples += 1

print(f"Samples {samples}, errors {errors}, over {over}")

# tester: https://learnmeabitcoin.com/tools/bitstarget/?bits=FFEE0021
