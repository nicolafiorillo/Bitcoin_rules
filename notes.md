# Napkin notes

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

bitcoin-cli getwalletinfo
bitcoin-cli listwallets

----

https://worldwithouteng.com/articles/make-your-rust-code-unit-testable-with-dependency-inversion/
https://www.youtube.com/watch?v=jf_ddGnum_4
https://docs.rs/syn/2.0.38/syn/struct.File.html
https://fettblog.eu/rust-enums-wrapping-errors/
https://google.github.io/comprehensive-rust/error-handling/converting-error-types-example.html
https://blog.orhun.dev/automated-rust-releases/
https://willcrichton.net/notes/k-corrset/

----

fn a_new_key() {
    let (secret, address) = wallet::key::new(flags::network::Network::Testnet);
    println!("secret: {:?}", secret);
    println!("address: {:?}", address);
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

fn get_seed_from_sys_time() -> u32 {
    let s = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    s.subsec_nanos()
}

fn get_seed_from_system() -> u32 {
    let mut rnd = File::open("/dev/random").unwrap();

    let mut buffer = [0u8; 4];
    rnd.read_exact(&mut buffer).unwrap();

    u32::from_le_bytes(buffer)
}
