#!/usr/bin/env python3

# WORK IN PROGRESS

# This script asks Bitcoin Core node for all blocks and write them in a relational db (PostgreSQL)

import os
import asyncio
from bitcoinrpc import BitcoinRPC
from progress.bar import Bar
from time import sleep
from postgres import Postgres

BITCOIN_NODE_HOST = "192.168.178.54"
BITCOIN_NODE_PORT = 8332
BITCOIN_RPC_USER = os.environ["BITCOIN_RPC_USER"]
BITCOIN_RPC_PASSWORD = os.environ["BITCOIN_RPC_PASSWORD"]
POSTGRES_CONNECTION_STRING = os.environ["POSTGRES_CONNECTION_STRING"]

RETRY = 12
RETRY_SLEEP = 10

FROM_BLOCK_HEIGHT = 0
TO_BLOCK_HEIGHT = 0
BLOCK_HEIGH_LENGTH = TO_BLOCK_HEIGHT - FROM_BLOCK_HEIGHT

def flush_tuple(file, current):
    file.write("{}, {}, 0x{}, {}\n".format(current[0], current[1], current[2], current[3]))
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
    db = Postgres(POSTGRES_CONNECTION_STRING)

    with Bar('Loading', fill='@', suffix='%(index)d/%(max)d - %(percent).1f%% - %(eta)ds', max=BLOCK_HEIGH_LENGTH) as bar:

        for i in range(FROM_BLOCK_HEIGHT, TO_BLOCK_HEIGHT):
            block = await read_block(i)

            bar.next()


if __name__ == "__main__":
    asyncio.run(main())


# CREATE TABLE public.blocks (
# 	height integer NOT NULL,
# 	id varchar NOT NULL,
# 	block bytea NULL,
# 	CONSTRAINT blocks_pk PRIMARY KEY (height),
# 	CONSTRAINT blocks_unique UNIQUE (id)
# );
# CREATE UNIQUE INDEX blocks_height_idx ON public.blocks (height);
# CREATE UNIQUE INDEX blocks_id_idx ON public.blocks (id);


# print(await rpc.getblockchaininfo())
# block = await rpc.getblock(hash, 1)
