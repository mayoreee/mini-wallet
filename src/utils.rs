use crate::Network;

/// Returns a BDK-specific Bitcoin Network
pub fn get_network(network: &Network) -> bdk::bitcoin::Network {
    match network {
        Network::Mainnet => bdk::bitcoin::Network::Bitcoin,
        Network::Testnet => bdk::bitcoin::Network::Testnet,
        Network::Signet => bdk::bitcoin::Network::Signet,
        Network::Regtest => bdk::bitcoin::Network::Regtest,
    }
}
