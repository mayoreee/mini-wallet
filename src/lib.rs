pub mod utils;

use base64;
use std::str::FromStr;

use bdk::bitcoin::consensus::serialize;
use bdk::bitcoin::Address;
use bdk::bitcoin::psbt::PartiallySignedTransaction;
use bdk::wallet::AddressIndex::New;
use bdk::wallet::AddressInfo;
use bdk::{FeeRate, SignOptions};
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

    /// Returns a Partially Signed Bitcoin Transaction (PSBT)
    pub fn create_tx(&self, recipient: &str, amount: u64) -> Result<String, bdk::Error> {
        let wallet = self.create_wallet()?;

        let recipient_address: Address = Address::from_str(recipient).unwrap();

        let (psbt, _details) = {
            let mut builder = wallet.build_tx();
            builder
                .add_recipient(recipient_address.script_pubkey(), amount)
                .enable_rbf()
                .fee_rate(FeeRate::from_sat_per_vb(1.0));
            builder.finish()?
        };

        // Encode PSBT as base64
        let psbt_encoded = base64::encode(&serialize(&psbt));

        Ok(psbt_encoded)
    }

    /// Returns a derived address using external descriptor
    pub fn get_address(&self) -> Result<AddressInfo, bdk::Error> {
        let wallet = self.create_wallet()?;
        let address: AddressInfo = wallet.get_address(New)?;
        Ok(address)
    }

    /// Signs a PSBT with all the wallet's signers
    pub fn sign_tx(&self, psbt: &mut str) -> Result<String, bdk::Error> {
      let wallet = self.create_wallet()?;

      let mut psbt: PartiallySignedTransaction = PartiallySignedTransaction::from_str(psbt)?;

      let finalized: bool = wallet.sign(&mut psbt, SignOptions::default())?;

      // Ensure psbt is finalized
      assert_eq!(finalized, true);

      // Encode PSBT as base64
      let psbt_encoded: String = base64::encode(&serialize(&psbt));

      Ok(psbt_encoded)
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

    #[test]
    fn test_create_tx() {
        let wallet: Wallet = Wallet {
      descriptor: String::from("wpkh([c258d2e4/84'/1'/0']tpubDDYkZojQFQjht8Tm4jsS3iuEmKjTiEGjG6KnuFNKKJb5A6ZUCUZKdvLdSDWofKi4ToRCwb9poe1XdqfUnP4jaJjCB2Zwv11ZLgSbnZSNecE/0/*)#l76duyv7"),
      change_descriptor: String::from("wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/1/*)"),
network: Network::Testnet
    };

        let recipient = wallet.get_address().unwrap().to_string();
        let result = wallet.create_tx(&recipient, 1_000);
        assert_eq!(result.is_ok(), true); // transaction should be created
    }

    #[test]
    fn test_sign_tx() {
        let wallet: Wallet = Wallet {
      descriptor: String::from("wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/0/*)"),
      change_descriptor: String::from("wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/1/*)"),
      network: Network::Testnet
    };
        let recipient = wallet.get_address().unwrap().to_string();
        let mut psbt = wallet.create_tx(&recipient, 1_000).unwrap();
        let result = wallet.sign_tx(&mut psbt);
        assert_eq!(result.is_ok(), true); // signed transaction should be created
    }
}
