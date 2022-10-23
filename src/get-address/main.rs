// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example generate_address --release
// In this example we will generate an address
// Rename `.env.example` to `.env` first

use std::env;

use dotenv::dotenv;
use iota_wallet::{account_manager::AccountManager, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // Create the account manager
    let manager = AccountManager::builder().finish().await?;

    // Get the account we generated with `01_create_wallet`
    let account = manager.get_account("Alice").await?;

    // Set the stronghold password
    manager
        .set_stronghold_password(&env::var("STRONGHOLD_PASSWORD").unwrap())
        .await?;

    let addresses = account.addresses().await?;
    for elem in addresses {
        println!("address: {}", elem.address().to_bech32());
    }

    Ok(())
}