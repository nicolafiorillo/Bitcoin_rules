#!/usr/bin/env python3

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

FILENAME = "fixtures/blocks.csv"
BLOCKS = [0, 1, 2015, 30240, 32255, 32256, 34271, 34272, 36287, 227836, 359875, 384548, 398364, 477120]
BLOCKS.sort()

async def read_block(i):
    block = None
    hash = None

    async with BitcoinRPC.from_config(f"http://{BITCOIN_NODE_HOST}:{BITCOIN_NODE_PORT}", (BITCOIN_RPC_USER, BITCOIN_RPC_PASSWORD)) as rpc:
        retry = RETRY
        while retry > 0:
            try:
                hash = await rpc.getblockhash(i)
                block = await rpc.getblock(hash, 0, 30)
                break
            except:
                retry -= 1
                sleep(RETRY_SLEEP)

    return (i, hash, block)

async def main():
        with Bar('Loading', fill='@', suffix='%(index)d/%(max)d - %(percent).1f%% - %(eta)ds', max=len(BLOCKS)) as bar:
            with open(FILENAME, "w") as file:
                # file.write("BLOCK_HEIGHT, BLOCK_ID, BLOCK\n")

                for i in BLOCKS:
                    (i, hash, block) = await read_block(i)
                    file.write(f"{i}, {hash}, {block}\n")
                    file.flush()

                    bar.next()

if __name__ == "__main__":
    asyncio.run(main())
