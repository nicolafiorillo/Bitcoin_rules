/*
    Functional verification for the difficulty adjustment algorithm.
    Currently it read the difficulty from a csv file that contains all difficulties
    for each epoch get a from Bitcoin Core node.
    Then verifies that the Bitcoin_rules! algorithm for difficulty is correct for each epoch.

    TODO: Currently it read blockchain blocks from a fixture file (`tests/fixtures/blocks.csv`)
    but it will be changed to read from a real node (Bitcoin_rules! or Bitcoin Core).
*/

extern crate brl;

#[cfg(test)]
mod verify_difficulty_test {

    use brl::{
        block::header::{adjust_target, bits_to_target, difficulty, target_to_bits},
        chain::header::get_header_by_height,
        flags::network::Network,
        std_lib::{fixture::load_fixture_file, vector::hex_string_to_u32},
    };

    #[test]
    pub fn verify_difficult_for_blocks_from_csv() {
        let difficulties_fixture = load_fixture_file("difficulty_by_block.csv");
        let difficulties = read_difficulties_from_fixture(&difficulties_fixture);

        for i in (0..difficulties.len() - 1).step_by(2) {
            let from = difficulties[i].from_block;
            let to = difficulties[i].to_block;

            let total_blocks = (to - from) / 2015;
            let mut current_block: u32 = 0;

            let mut start: u32 = from;
            let mut end: u32 = start + 2015;

            while current_block < total_blocks {
                let expected_difficulty = if current_block == (total_blocks - 1) {
                    difficulties[i + 1].difficulty
                } else {
                    difficulties[i].difficulty
                };

                let first_block_header = get_header_by_height(&start, Network::Mainnet).unwrap();
                let last_block_header = get_header_by_height(&end, Network::Mainnet).unwrap();

                let new_target = adjust_target(&first_block_header, &last_block_header);
                let new_bits = target_to_bits(new_target.clone());

                let check_target = bits_to_target(new_bits);

                let difficulty = difficulty(check_target);

                assert_eq!(expected_difficulty, difficulty);

                current_block = current_block + 1;

                start = end + 1;
                end = start + 2015;
            }
        }
    }

    struct DifficultyBlockFixture {
        pub from_block: u32,
        pub to_block: u32,
        pub bits: u32,
        pub difficulty: f64,
    }

    fn read_difficulties_from_fixture(fixture: &str) -> Vec<DifficultyBlockFixture> {
        let content = std::fs::read_to_string(fixture).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        let mut difficulties = Vec::<DifficultyBlockFixture>::new();

        for line in lines {
            if line.is_empty() {
                continue;
            }

            let h: Vec<&str> = line.split(',').collect();
            let from_block = h[0].parse::<u32>().unwrap();
            let to_block = h[1].parse::<u32>().unwrap();

            let bits = hex_string_to_u32(h[2]).unwrap();
            let difficulty = h[3].parse::<f64>().unwrap();

            difficulties.push(DifficultyBlockFixture {
                from_block,
                to_block,
                bits,
                difficulty,
            });
        }

        difficulties
    }
}
