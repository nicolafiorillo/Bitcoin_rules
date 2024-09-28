#!/usr/bin/env python3

# This script ask node for all blocks and write the difficulty and bits when changed to a csv file.

import os
import asyncio
from bitcoinrpc import BitcoinRPC
from progress.bar import Bar
from time import sleep

BITCOIN_NODE_HOST = "192.168.178.54"
BITCOIN_NODE_PORT = 8332
BITCOIN_RPC_USER = os.environ["BITCOIN_RPC_USER"]
BITCOIN_RPC_PASSWORD = os.environ["BITCOIN_RPC_PASSWORD"]

RETRY = 12
RETRY_SLEEP = 10

FILENAME = "./difficulty_by_block.csv"
FILENAME_LAST = "./difficulty_by_block.last"

FROM_BLOCK_HEIGHT = 0
TO_BLOCK_HEIGHT = 840236
BLOCK_HEIGH_LENGTH = TO_BLOCK_HEIGHT - FROM_BLOCK_HEIGHT
LAST_CURRENT=None

def flush_tuple(file, current):
    file.write("{},{},{},{}\n".format(current[0], current[1], current[2], current[3]))
    file.flush()

async def read_block(i):
    block = None
    async with BitcoinRPC.from_config(f"http://{BITCOIN_NODE_HOST}:{BITCOIN_NODE_PORT}", (BITCOIN_RPC_USER, BITCOIN_RPC_PASSWORD)) as rpc:
        retry = RETRY
        while retry > 0:
            try:
                hash = await rpc.getblockhash(i)
                block = await rpc.getblock(hash, 1, 30)
                break
            except:
                retry -= 1
                sleep(RETRY_SLEEP)

    return block

async def main():
        with Bar('Loading', fill='@', suffix='%(index)d/%(max)d - %(percent).1f%% - %(eta)ds', max=BLOCK_HEIGH_LENGTH) as bar:
            with open(FILENAME_LAST, "w") as file_last:
                with open(FILENAME, "a") as file:
                    # file.write("FROM_BLOCK_HEIGHT, TO_BLOCK_HEIGHT, BITS, DIFFICULTY\n")

                    current = LAST_CURRENT

                    for block_index in range(FROM_BLOCK_HEIGHT, TO_BLOCK_HEIGHT + 1):
                        block = await read_block(block_index)

                        if current is None:
                            current = (block_index, block_index, block["bits"], block["difficulty"])
                        elif block["bits"] != current[2]:
                            flush_tuple(file, current)
                            current = (block_index, block_index, block["bits"], block["difficulty"])
                        elif block["bits"] == current[2]:
                            current = (current[0], block_index, current[2], current[3])
                    
                        file_last.write(str(current))
                        file_last.seek(0)
                        bar.next()

                    flush_tuple(file, current)


if __name__ == "__main__":
    asyncio.run(main())


# print(await rpc.getblockchaininfo())
# block = await rpc.getblock(hash, 1)
