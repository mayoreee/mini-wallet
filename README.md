# mini-wallet

Rust implementation of a minimal Bitcoin wallet

# Installation

Add to dependencies in `Cargo.toml` file

```rust
[dependencies]
mini-wallet = { git = "https://github.com/mayoreee/mini-wallet", branch = "master" }
```

# Usage

The wallet features methods for the following:

- send bitcoin (full operation)
- send bitcoin (watch-only operation)

## Send Bitcoin (full operation)

```rust
use mini_wallet::{ Network, Wallet };

fn main() {
    let descriptor = "<Insert descriptor>";
    let change_descriptor = "<Insert change descriptor>";

    let recipient = "<Insert recipient address>"; // transaction recipient address
    let amount = 1_000; // amount in satoshis

    // Create a Wallet
    let wallet = Wallet {
        descriptor: String::from(descriptor),
        change_descriptor: String::from(change_descriptor),
        network: Network::Testnet,
    };

    // Send Bitcoin to recipient
    let txid = wallet.send_btc(recipient, amount).unwrap();

    println!("Transaction ID: {:?}", txid);
}
```

## Send Bitcoin (watch-only operation)

```rust
use mini_wallet::{ Network, Wallet };

fn main() {
    let descriptor = "<Insert descriptor>";
    let change_descriptor = "<Insert change descriptor>";

    let recipient = "<Insert recipient address>"; // transaction recipient address
    let amount = 1_000; // amount in satoshis

    // Create a Wallet
    let wallet = Wallet {
        descriptor: String::from(descriptor),
        change_descriptor: String::from(change_descriptor),
        network: Network::Testnet,
    };

    // Create a PSBT
    let psbt = wallet.create_tx(recipient, amount).unwrap();
    println!(psbt);
    
    /**
     *  Note: The PSBT (Partially Signed Bitcoin Transaction) is provided in base64 encoded format.
     *        It is to be signed with an external wallet.
     */

    let psbt_signed = "<Insert the signed PSBT>"

    let txid = wallet.broadcast_tx(psbt_signed).unwrap();

    println!("Transaction ID: {:?}", txid);
}

```
