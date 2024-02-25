use ethers::{
    core::rand,
    signers::{coins_bip39::English, MnemonicBuilder, Signer},
};
use eyre::Result;
use hex;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let wallet = if args.len() > 1 {
        // Use provided mnemonic phrase
        let phrase = &args[1];
        let index = 0u32;

        // Access mnemonic phrase with derivation path: m/44'/60'/0'/0/{index}
        MnemonicBuilder::<English>::default()
            .phrase(phrase)
            .index(index)?
            .build()?
    } else {
        // Generate a random wallet (12 word phrase) at custom derivation path
        let mut rng = rand::thread_rng();
        MnemonicBuilder::<English>::default()
            .word_count(12)
            .derivation_path("m/44'/60'/0'/0/0")?
            .build_random(&mut rng)?
    };

    let priv_key = hex::encode(wallet.signer().to_bytes());
    let addr = wallet.address();

    if args.len() > 1 {
        eprintln!("Wallet from provided mnemonic: {wallet:?}");
    } else {
        eprintln!("Random wallet: {wallet:?}");
    }
    eprintln!("Address: {addr}");
    eprintln!("Privkey: {priv_key:?}");

    Ok(())
}