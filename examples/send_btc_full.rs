use mini_wallet::{Network, Wallet};

fn main() {
    let descriptor = "wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/0/*)";
    let change_descriptor = "wpkh([e9824965/84'/1'/0']tprv8fvem7qWxY3SGCQczQpRpqTKg455wf1zgixn6MZ4ze8gRfHjov5gXBQTadNfDgqs9ERbZZ3Bi1PNYrCCusFLucT39K525MWLpeURjHwUsfX/1/*)";

    let recipient = "tb1ql7w62elx9ucw4pj5lgw4l028hmuw80sndtntxt"; // transaction recipient address
    let amount = 1_000; // amount in satoshis

    let wallet = Wallet {
        descriptor: String::from(descriptor),
        change_descriptor: String::from(change_descriptor),
        network: Network::Testnet,
    };

    let txid = wallet.send_btc(recipient, amount).unwrap();

    println!("Transaction ID: {:?}", txid);
}
