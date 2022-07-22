use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserCredit {
    pub id: String,
    pub username: String,
    pub user_id: String,
    pub credits: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UserCreditUpdateInfo {
    pub credit: i32,
}
