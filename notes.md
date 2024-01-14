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

## bitcoin-cli commands

loadwallet <name>
listwallets
getwalletinfo
getbalance
listtransactions
listunspent
gettransaction <txid>
decoderawtransaction <hex_tx>
getblockchaininfo
getblockcount
getnewaddress
validateaddress <hex_address>
sendrawtransaction <hex_tx>
getaddressinfo <hex_address>

generate <num_blocks>
settxfee 0.001
getmininginfo

sendtoaddress <hex_address> <amount>

## bitcoincli RPC

curl --user nicola:nicola --data-binary '{"jsonrpc": "1.0", "id": "curltest", "method": "getblockchaininfo", "params": []}' -H 'content-type: text/plain;' http://@192.168.178.54:8332/

----
openssl rand -hex 32

----

## print op_return data
echo <hex_data> | xxd -p -r

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

OP_BOOLAND 	154 	0x9a 	a b 	out 	If both a and b are not 0, the output is 1. Otherwise 0.
OP_BOOLOR 	155 	0x9b 	a b 	out 	If a or b is not 0, the output is 1. Otherwise 0.
OP_NUMEQUAL 	156 	0x9c 	a b 	out 	Returns 1 if the numbers are equal, 0 otherwise.
OP_NUMEQUALVERIFY 	157 	0x9d 	a b 	Nothing / fail 	Same as OP_NUMEQUAL, but runs OP_VERIFY afterward.
OP_NUMNOTEQUAL 	158 	0x9e 	a b 	out 	Returns 1 if the numbers are not equal, 0 otherwise.
OP_LESSTHAN 	159 	0x9f 	a b 	out 	Returns 1 if a is less than b, 0 otherwise.
OP_GREATERTHAN 	160 	0xa0 	a b 	out 	Returns 1 if a is greater than b, 0 otherwise.
OP_LESSTHANOREQUAL 	161 	0xa1 	a b 	out 	Returns 1 if a is less than or equal to b, 0 otherwise.
OP_GREATERTHANOREQUAL 	162 	0xa2 	a b 	out 	Returns 1 if a is greater than or equal to b, 0 otherwise.
OP_MIN 	163 	0xa3 	a b 	out 	Returns the smaller of a and b.
OP_MAX 	164 	0xa4 	a b 	out 	Returns the larger of a and b.
OP_WITHIN 	165 	0xa5 	x min max 	out 	Returns 1 if x is within the specified range (left-inclusive), 0 otherwise. 
