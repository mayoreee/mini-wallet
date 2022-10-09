#[derive(Debug, Clone, PartialEq)]
pub enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wallet {
    pub descriptor: String,
    pub change_descriptor: String,
    pub network: Network,
}

impl Wallet {
    
}