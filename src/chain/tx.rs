// For now we will use the tx.rs file to mocking fetching transaction from node

use std::collections::HashMap;

use once_cell::sync::Lazy;
use rug::Integer;

use crate::{
    flags::network::Network,
    std_lib::{integer_ex::IntegerEx, vector::string_to_bytes},
    transaction::tx::Tx,
};

#[derive(Debug)]
pub enum ChainError {
    TransactionNotFound,
}

fn get_id_to_transaction(id: &str, tx: &str, network: Network) -> (Integer, Tx) {
    let s = string_to_bytes(tx).unwrap();
    let tx = Tx::from_serialized(&s, network).unwrap();
    let id = IntegerEx::from_hex_str(id);

    (id, tx)
}

pub static MAINNET: Lazy<HashMap<Integer, Tx>> = Lazy::new(|| {
    let mut h: HashMap<Integer, Tx> = HashMap::new();

    let (id, tx) = get_id_to_transaction("ee51510d7bbabe28052038d1deb10c03ec74f06a79e21913c6fcf48d56217c87", "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600", Network::Mainnet);
    h.insert(id, tx);

    let (id, tx) = get_id_to_transaction("9e067aedc661fca148e13953df75f8ca6eada9ce3b3d8d68631769ac60999156", "0100000001c228021e1fee6f158cc506edea6bad7ffa421dd14fb7fd7e01c50cc9693e8dbe02000000fdfe0000483045022100c679944ff8f20373685e1122b581f64752c1d22c67f6f3ae26333aa9c3f43d730220793233401f87f640f9c39207349ffef42d0e27046755263c0a69c436ab07febc01483045022100eadc1c6e72f241c3e076a7109b8053db53987f3fcc99e3f88fc4e52dbfd5f3a202201f02cbff194c41e6f8da762e024a7ab85c1b1616b74720f13283043e9e99dab8014c69522102b0c7be446b92624112f3c7d4ffc214921c74c1cb891bf945c49fbe5981ee026b21039021c9391e328e0cb3b61ba05dcc5e122ab234e55d1502e59b10d8f588aea4632102f3bd8f64363066f35968bd82ed9c6e8afecbd6136311bb51e91204f614144e9b53aeffffffff05a08601000000000017a914081fbb6ec9d83104367eb1a6a59e2a92417d79298700350c00000000001976a914677345c7376dfda2c52ad9b6a153b643b6409a3788acc7f341160000000017a914234c15756b9599314c9299340eaabab7f1810d8287c02709000000000017a91469be3ca6195efcab5194e1530164ec47637d44308740420f00000000001976a91487fadba66b9e48c0c8082f33107fdb01970eb80388ac00000000", Network::Mainnet);
    h.insert(id, tx);

    let (id, tx) = get_id_to_transaction("d37f9e7282f81b7fd3af0fde8b462a1c28024f1d83cf13637ec18d03f4518feb", "0100000001b74780c0b9903472f84f8697a7449faebbfb1af659ecb8148ce8104347f3f72d010000006b483045022100bb8792c98141bcf4dab4fd4030743b4eff9edde59cec62380c60ffb90121ab7802204b439e3572b51382540c3b652b01327ee8b14cededc992fbc69b1e077a2c3f9f0121027c975c8bdc9717de310998494a2ae63f01b7a390bd34ef5b4c346fa717cba012ffffffff01a627c901000000001976a914af24b3f3e987c23528b366122a7ed2af199b36bc88ac00000000", Network::Mainnet);
    h.insert(id, tx);

    let (id, tx) = get_id_to_transaction("75d7454b7010fa28b00f16cccb640b1756fd6e357c03a3b81b9d119505f47b56", "010000000367d54ded4c43569acbc213073fc63bfc49bf420391f0ab304758b16600a8ea88010000006a4730440220404b3bb28af45437c989328122aa6f4462021a0a2d4f20141ebe84e80edd72e202204184dd9d833d57246eaeed39021e9ab8c0546f3270bd9d2fc138a4bf161ea2310121039550662b907f788cc96708dc017aee0d407b74427f11e656b87f84146337f183feffffff5edf7dbc586b5fddace63a6614f5a731787c104d3c1c9225c4542db067d4296d010000006b483045022100b2335adb91e1ac3bb4e0479b54a9e7d4b765d9b646ca71e2547776c4e7e6bdfb02201fa8aaa4d2557768329befd61d4abda95668f88065df6eac6076e3e123c121eb012103b80229ec7a62793132ff432be0ecf21bca774ade18af7eaf2215febad0c4321ffeffffffdfa74eb50768daeb4beca2ca83d1732128d2439f9df9508efc8f7820718b4ae1000000006a47304402204818b29bed4a8ea4eb383f996389866a732b44d98f6342ecc25007ca472526fb0220496ed1213d63b7686f6936940e8f566f291bab211e6600c0f71e3659787b91fc0121036a30f9e6f645191c6216f84c21ae3b4f0aca0c4be987889276089cf9ef7a89d6feffffff028deb0f00000000001976a914cd0b3a22cd16e182291aa2708c41cb38de5a330788acc0e1e400000000001976a91424505f6d2f0fe7c4a3f4af32f50506034d89095d88ac43430600", Network::Mainnet);
    h.insert(id, tx);

    let (id, tx) = get_id_to_transaction("45f3f79066d251addc04fd889f776c73afab1cb22559376ff820e6166c5e3ad6", "01000000012aa311f7789d362ceb2d802a98a703e0ac44815c021293633b80d08e67232e36010000006a4730440220142d8810ab29cac9199e6b570d47bd5ee402accf9d754cfa7de9b2e84e3997b402207a7d8c77c6a721bc64dba39eabe23e915c979683e621921c243bb35b3f538dfb01210371cb7d04e95471c4ea5c200e8c4729608754c74bee4e289bd66f431482407ec8feffffff02a08601000000000017a914fc7d096f19063ece361e2b309ec4da41fe4d789487f2798e00000000001976a914311b232c3400080eb2636edb8548b47f6835be7688ac31430600", Network::Mainnet);
    h.insert(id, tx);

    // a Satoshi Nakamoto coinbase transaction used in first transaction
    let (id, tx) = get_id_to_transaction("0437cd7f8525ceed2324359c2d0ba26006d92d856a9c20fa0241106ee5a597c9", "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704ffff001d0134ffffffff0100f2052a0100000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000", Network::Mainnet);
    h.insert(id, tx);

    // first transaction ever: from Satoshi Nakamoto to Hal Finney
    let (id, tx) = get_id_to_transaction("f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16", "0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000", Network::Mainnet);
    h.insert(id, tx);

    h
});

pub static TESTNET: Lazy<HashMap<Integer, Tx>> = Lazy::new(|| HashMap::from([]));

pub fn get_transaction(transaction_id: &Integer, network: Network) -> Result<&Tx, ChainError> {
    let h = match network {
        Network::Testnet => &*TESTNET,
        Network::Mainnet => &*MAINNET,
    };

    let tx = h.get(transaction_id).ok_or(ChainError::TransactionNotFound)?;

    Ok(tx)
}
