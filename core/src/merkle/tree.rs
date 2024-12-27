use crate::{hashing::hash256::Hash256, std_lib::std_result::StdResult};

pub fn merkle_root(hashes: Vec<Hash256>) -> StdResult<Hash256> {
    if hashes.len() == 0 {
        return Err("Merkle root: hashes must not be empty")?;
    }

    let mut level = hashes;
    while level.len() > 1 {
        level = merkle_parent_level(level)?;
    }

    Ok(level[0])
}

fn merkle_parent(left: Hash256, right: Hash256) -> Hash256 {
    let combined: Vec<u8> = [left.0.as_slice(), right.0.as_slice()].concat();
    Hash256::calc(&combined)
}

fn merkle_parent_level(mut level: Vec<Hash256>) -> StdResult<Vec<Hash256>> {
    if level.len() == 0 {
        return Err("Merkle parent: hashes must not be empty")?;
    }

    if level.len() % 2 == 1 {
        level.push(level[level.len() - 1]);
    }

    let mut parent_level = Vec::with_capacity(level.len() / 2);

    for chunk in level.chunks(2) {
        let parent = merkle_parent(chunk[0], chunk[1]);
        parent_level.push(parent);
    }

    Ok(parent_level)
}

#[cfg(test)]
mod tree_test {

    use super::{merkle_parent, merkle_parent_level, merkle_root};
    use crate::{hashing::hash256::Hash256, std_lib::vector::hex_string_to_bytes};

    #[test]
    fn verify_merkle_parent() {
        let left = hex_string_to_bytes("c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5").unwrap();
        let right = hex_string_to_bytes("c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5").unwrap();

        let parent = merkle_parent(
            Hash256::new(left.try_into().unwrap()),
            Hash256::new(right.try_into().unwrap()),
        );

        let e = hex_string_to_bytes("8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd").unwrap();
        let expected = Hash256::new(e.try_into().unwrap());

        assert!(parent == expected);
    }

    fn hashes_str_to_hash256(s: Vec<&str>) -> Vec<Hash256> {
        s.iter()
            .map(|s| Hash256::new(hex_string_to_bytes(s).unwrap().try_into().unwrap()))
            .collect()
    }

    #[test]
    fn verify_merkle_parent_level() {
        let str_hashes = vec![
            "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
            "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
            "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
            "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
        ];

        let hashes = hashes_str_to_hash256(str_hashes);

        let str_expected = vec![
            "8b30c5ba100f6f2e5ad1e2a742e5020491240f8eb514fe97c713c31718ad7ecd",
            "7f4e6f9e224e20fda0ae4c44114237f97cd35aca38d83081c9bfd41feb907800",
            "3ecf6115380c77e8aae56660f5634982ee897351ba906a6837d15ebc3a225df0",
        ];

        let parent = merkle_parent_level(hashes).unwrap();

        let expected = hashes_str_to_hash256(str_expected);

        assert!(parent == expected);
    }

    #[test]
    fn verify_merkle_root() {
        let str_hashes = vec![
            "c117ea8ec828342f4dfb0ad6bd140e03a50720ece40169ee38bdc15d9eb64cf5",
            "c131474164b412e3406696da1ee20ab0fc9bf41c8f05fa8ceea7a08d672d7cc5",
            "f391da6ecfeed1814efae39e7fcb3838ae0b02c02ae7d0a5848a66947c0727b0",
            "3d238a92a94532b946c90e19c49351c763696cff3db400485b813aecb8a13181",
            "10092f2633be5f3ce349bf9ddbde36caa3dd10dfa0ec8106bce23acbff637dae",
            "7d37b3d54fa6a64869084bfd2e831309118b9e833610e6228adacdbd1b4ba161",
            "8118a77e542892fe15ae3fc771a4abfd2f5d5d5997544c3487ac36b5c85170fc",
            "dff6879848c2c9b62fe652720b8df5272093acfaa45a43cdb3696fe2466a3877",
            "b825c0745f46ac58f7d3759e6dc535a1fec7820377f24d4c2c6ad2cc55c0cb59",
            "95513952a04bd8992721e9b7e2937f1c04ba31e0469fbe615a78197f68f52b7c",
            "2e6d722e5e4dbdf2447ddecc9f7dabb8e299bae921c99ad5b0184cd9eb8e5908",
            "b13a750047bc0bdceb2473e5fe488c2596d7a7124b4e716fdd29b046ef99bbf0",
        ];

        let hashes = hashes_str_to_hash256(str_hashes);

        let root = merkle_root(hashes).unwrap();

        let str_expected = "acbcab8bcc1af95d8d563b77d24c3d19b18f1486383d75a5085c4e86c86beed6";
        let expected = Hash256::new(hex_string_to_bytes(str_expected).unwrap().try_into().unwrap());

        assert!(root == expected);
    }
}
