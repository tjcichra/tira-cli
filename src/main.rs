use std::collections::HashMap;

use crate::models::User;

mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:8000/users")
        .await?
        .json::<Vec<User>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
