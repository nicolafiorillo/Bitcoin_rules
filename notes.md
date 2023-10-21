https://blockchain.info/rawtx/ee51510d7bbabe28052038d1deb10c03ec74f06a79e21913c6fcf48d56217c87?cors=true&format=hex
https://blockchain.info/rawtx/d37f9e7282f81b7fd3af0fde8b462a1c28024f1d83cf13637ec18d03f4518feb?cors=true&format=hex

https://blockchain.info/rawtx/d276abe15791941649c3ca8425d79167cc1cf801f83aa99753fe7f42740c0f23?cors=true&format=hex
https://blockchain.info/rawtx/728e24b2e7dd137e574c433a8db08ac2aa0bf0588ad7716e4c5a7da45dbb5933?cors=true&format=hex


https://blockchain.info/rawtx/ee51510d7bbabe28052038d1deb10c03ec74f06a79e21913c6fcf48d56217c87?cors=true&format=hex

https://blockchain.info/rawtx/0437CD7F8525CEED2324359C2D0BA26006D92D856A9C20FA0241106EE5A597C9?cors=true&format=hex
https://blockchain.info/rawtx/F4184FC596403B9D638783CF57ADFE4C75C605F6356FBC91338530E9831E9E16?cors=true&format=hex

https://blockstream.info/api/tx/f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16/hex

https://blockstream.info/testnet/api/tx/2ad00c8e79a0c62c613d51e4669a14a4a94302e487be38ce1316a2ecc705c646/hex


----


fn a_new_key() {
    let (secret, address) = wallet::key::new(flags::network::Network::Testnet);
    println!("secret: {:?}", secret);
    println!("address: {:?}", address);
}

fn a_new_transaction() -> Tx {
    let network = Network::Testnet;

    let previous_transaction_id =
        Integer::from_hex_str("66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c");
    let previous_transaction = chain::tx::get_transaction(&previous_transaction_id, network).unwrap();

    let output_transaction_0 = previous_transaction.get_output(0).unwrap();
    let script_pub_key_0 = output_transaction_0.script_pub_key.clone();

    let output_transaction_1 = previous_transaction.get_output(1).unwrap();
    let script_pub_key_1 = output_transaction_1.script_pub_key.clone();

    let private_key_0 =
        Integer::from_dec_str("421788365705557317699661811707659433049257527084948635109995507081033905");
    let private_key_1 =
        Integer::from_dec_str("1029357723937880961141771287078141638280703752019825886047717249989513591");

    let tx_in_0 = wallet::transaction::new_tx_in(
        "66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c",
        0,
        network,
    );

    let tx_in_1 = wallet::transaction::new_tx_in(
        "66142ec32e651f7f5dc0c23cfc4e7a43bc4ba2971196f88ad5ff27477cf57d8c",
        1,
        network,
    );

    let tx_out = wallet::transaction::new_out(4661, "muekgXwwwbFJTVq1JTbi7Lrwi7fyWY8PEZ", network).unwrap();

    let mut tx = wallet::transaction::new_tx(&[tx_in_0.clone(), tx_in_1.clone()], &[tx_out], network);

    let script_0 = signing::generate_input_signature(&tx, 0, &private_key_0, script_pub_key_0).unwrap();
    tx.substitute_script(0, script_0);

    let script_1 = signing::generate_input_signature(&tx, 1, &private_key_1, script_pub_key_1).unwrap();
    tx.substitute_script(1, script_1);

    tx
}

fn a_satoshi_transaction() {
    let satoshi_transaction_id: Integer =
        Integer::from_hex_str("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16");
    // let satoshi_transaction_id: Integer =
    //     Integer::from_hex_str("0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9");

    let satoshi_transaction = chain::tx::get_transaction(&satoshi_transaction_id, Network::Mainnet).unwrap();

    println!();
    println!("Satoshi transaction");
    println!("{:}", satoshi_transaction);

    println!(
        "is valid: {:}",
        transaction::verification::validate(satoshi_transaction).unwrap()
    );
}

fn a_testnet_transaction() {
    let tx_id: Integer = Integer::from_hex_str("2ad00c8e79a0c62c613d51e4669a14a4a94302e487be38ce1316a2ecc705c646");

    let tx = chain::tx::get_transaction(&tx_id, Network::Testnet).unwrap();

    println!("Testnet transaction");
    println!("{:}", tx);
}
