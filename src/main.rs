use ethers::{
    core::rand,
    signers::{coins_bip39::English, MnemonicBuilder},
};
use eyre::Result;

fn main() -> Result<()> {
    // let phrase =
    //     "setup desert impact cabbage special agree daughter lady midnight crumble saddle segment";
    // let index = 0u32;

    // // Access mnemonic phrase with password
    // // Child key at derivation path: m/44'/60'/0'/0/{index}
    // let wallet = MnemonicBuilder::<English>::default()
    //     .phrase(phrase)
    //     .index(index)?
    //     .build()?;

    // eprintln!("Wallet: {wallet:?}");

    // Generate a random wallet (24 word phrase) at custom derivation path
    let mut rng = rand::thread_rng();
    let wallet = MnemonicBuilder::<English>::default()
        .word_count(12)
        .derivation_path("m/44'/60'/0'/0/0")?
        // Optionally add this if you want the generated mnemonic to be written
        // to a file
        .write_to("./")
        .build_random(&mut rng)?;

    eprintln!("Random wallet: {wallet:?}");

    Ok(())
}
