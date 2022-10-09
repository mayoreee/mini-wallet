pub mod utils;

use bdk::{
    blockchain::ElectrumBlockchain, database::MemoryDatabase, electrum_client::Client, SyncOptions,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wallet {
    pub descriptor: String,        // wallet output descriptor
    pub change_descriptor: String, // change output descriptor
    pub network: Network,          // network used by the wallet
}

impl Wallet {
    /// Returns a newly created BDK Wallet
    fn create_wallet(&self) -> Result<bdk::Wallet<MemoryDatabase>, bdk::Error> {
        let descriptor: &str = &self.descriptor;
        let change_descriptor: &str = &self.change_descriptor;

        let network = utils::get_network(&self.network);

        let client: Client = Client::new("ssl://electrum.blockstream.info:60002")?;
        let blockchain: ElectrumBlockchain = ElectrumBlockchain::from(client);

        // Create a new BDK Wallet
        let bdk_wallet = bdk::Wallet::new(
            descriptor,
            Some(change_descriptor),
            network,
            MemoryDatabase::default(),
        )?;

        // Sync wallet's internal database with the blockchain
        bdk_wallet.sync(&blockchain, SyncOptions::default())?;

        Ok(bdk_wallet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet() {
        let wallet: Wallet = Wallet {
      descriptor: String::from("wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/0/*)"),
      change_descriptor: String::from("wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/1/*)"),
      network: Network::Testnet
    };

        let result = wallet.create_wallet();

        assert_eq!(result.is_ok(), true); // wallet should be created
        assert_eq!(
            result.as_ref().unwrap().network(),
            bdk::bitcoin::Network::Testnet
        ); // wallet should use the correct bitcoin network
    }
}
