use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    email_address: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}