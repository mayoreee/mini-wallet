use mini_wallet::{Network, Wallet};

#[test]
fn test_send_btc() {
    let wallet = Wallet {
descriptor: String::from("wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/0/*)"),
change_descriptor: String::from("wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/1/*)"),
network: Network::Testnet
};
    let recipient = wallet.get_address().unwrap().to_string();
    let amount = 1_000;

    let txid = wallet.send_btc(&recipient, amount);

    assert!(txid.is_ok()); // Transaction should be broadcasted to the network
}
