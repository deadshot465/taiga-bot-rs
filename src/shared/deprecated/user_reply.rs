use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserReply {
    pub user: u64,
    pub messages: Vec<String>,
}
