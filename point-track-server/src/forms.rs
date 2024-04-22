use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}
