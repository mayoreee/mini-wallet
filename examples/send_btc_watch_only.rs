use mini_wallet::{Network, Wallet};
use std::io;

fn main() {
    // Replace the descriptor with those from a wallet you control
    let descriptor = "wpkh([bf173f25/84'/1'/0'/0/1]03ab22671edf2aa1d9d3351de5082b559c1af71c3a5227b139eb8a683782326f33)#v4eu0sl7";
    let change_descriptor = "wpkh([bf173f25/84'/1'/0'/0/1]03ab22671edf2aa1d9d3351de5082b559c1af71c3a5227b139eb8a683782326f33)#v4eu0sl7";

    let recipient = "tb1qh282uml3jkwu7c9puts00ngfzmvgrmmqaeuk3r"; // transaction recipient
    let amount = 1_000; // amount in satoshis

    let wallet = Wallet {
        descriptor: String::from(descriptor),
        change_descriptor: String::from(change_descriptor),
        network: Network::Testnet,
    };

    let psbt = wallet.create_tx(recipient, amount).unwrap();

    println!(
        "The PSBT data can be found below. Copy it and sign with your external wallet \n\n{}",
        psbt
    );

    println!("\n\nPaste the signed PSBT below\n\n");

    let mut psbt_signed = String::new();

    io::stdin()
        .read_line(&mut psbt_signed)
        .expect("Failure reading line");

    let psbt_signed = psbt_signed.trim().to_string();

    let txid = wallet.broadcast_tx(&psbt_signed).unwrap();

    println!("Transaction ID: {:?}", txid);
}
