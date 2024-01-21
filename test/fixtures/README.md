# Fixtures

Tools to create fixture set for testing `Bitcoin_rules!`.

Install dependencies with:

`pip install -r requirements.txt`

# `difficulty_by_block.py`

Generates `./difficulty_by_block.csv` querying Bitcoin Core node.

Result file containf all epochs (2016 blocks) and the difficulty for each block in the epoch.

# `generate_blocks.py`

Generates `./blocks.csv` querying Bitcoin Core node.

Result file contains block height, block id and the block itself.

# `import_blocks_from_bc_node.py`

Work in progress.


