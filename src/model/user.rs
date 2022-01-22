use derive_getters::Getters;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Getters)]
pub struct User {
    id: String,
    ip: String,
    token: u64,
}

pub fn create_user(user_id: &str, user_ip: &str) -> User {
    let token = calc_user_token(user_id, user_ip);
    User {
        id: user_id.to_string(),
        ip: user_ip.to_string(),
        token,
    }
}

pub fn calc_user_token(user_id: &str, user_ip: &str) -> u64 {
    let mut s = DefaultHasher::new();
    let origin_str = format!("{}{}-42", user_id, user_ip);
    s.write(origin_str.as_bytes());
    s.finish()
}
