use zksync::{
    self,
    signer::Signer,
    wallet::Wallet,
    zksync_types::{L2ChainId, PackedEthSignature, H256},
};
use zksync_eth_signer::PrivateKeySigner;

const CHAIN: u16 = 280;
const RPC_URL: &str = "https://testnet.era.zksync.dev";
// const RPC_URL: &str = "https://testnet.era.zksync.dev:443";

fn main() {
    let mut eth_private_key = H256::default();
    eth_private_key.randomize();

    let eth_signer = PrivateKeySigner::new(eth_private_key);
    let address_from_pk = PackedEthSignature::address_from_private_key(&eth_private_key).unwrap();
    let signer = Signer::new(eth_signer, address_from_pk, L2ChainId(CHAIN));

    // getting port error retrieving this wallet, if no port provided
    let wallet = Wallet::with_http_client(RPC_URL, signer);
    println!("{:#?} wallet", wallet);
}
