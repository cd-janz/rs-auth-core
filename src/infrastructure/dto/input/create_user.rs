use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDTO {
    first_name: String,
    last_name: String,
    email: String,
    usename: Option<String>,
    password: String,
}
