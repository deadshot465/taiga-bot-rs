use crate::shared::services::HTTP_CLIENT;
use crate::shared::structs::authentication::{login, AUTHENTICATION};
use crate::shared::structs::config::configuration::CONFIGURATION;
use crate::shared::structs::record::user_credit::{UserCredit, UserCreditUpdateInfo};
use reqwest::StatusCode;

pub async fn add_user_credit(user_id: u64, user_name: &str, amount: i32) -> anyhow::Result<()> {
    get_user_credit(user_id, user_name).await?;

    let server_endpoint = CONFIGURATION
        .get()
        .map(|c| c.server_endpoint.as_str())
        .unwrap_or_default();

    let request_data = UserCreditUpdateInfo { credit: amount };

    let response = HTTP_CLIENT
        .patch(format!("{}/{}/{}/plus", server_endpoint, "credit", user_id))
        .bearer_auth(AUTHENTICATION.read().await.token.clone())
        .json(&request_data)
        .send()
        .await?;

    match response.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Error when adding user's credit: {}", e)),
    }
}

pub async fn get_user_credit(user_id: u64, user_name: &str) -> anyhow::Result<UserCredit> {
    login().await?;

    let server_endpoint = CONFIGURATION
        .get()
        .map(|c| c.server_endpoint.as_str())
        .unwrap_or_default();

    let response = HTTP_CLIENT
        .get(format!("{}/{}/{}", server_endpoint, "credit", user_id))
        .bearer_auth(AUTHENTICATION.read().await.token.clone())
        .send()
        .await?;

    let response_status = response.status();
    match response_status {
        StatusCode::NOT_FOUND => create_user(server_endpoint, user_id, user_name).await,
        StatusCode::INTERNAL_SERVER_ERROR => Err(anyhow::anyhow!(
            "Internal server error: {}",
            response.text().await?
        )),
        StatusCode::OK => Ok(response.json().await?),
        _ => Err(anyhow::anyhow!(
            "An unknown error occurred when getting user's credit: {} - {}",
            response_status,
            response.text().await?
        )),
    }
}

async fn create_user(
    server_endpoint: &str,
    user_id: u64,
    user_name: &str,
) -> anyhow::Result<UserCredit> {
    login().await?;
    let response = HTTP_CLIENT
        .get(format!("{}{}", server_endpoint, "/credit"))
        .bearer_auth(AUTHENTICATION.read().await.token.clone())
        .send()
        .await?;

    match response.error_for_status() {
        Ok(res) => {
            let mut user_credits = res.json::<Vec<UserCredit>>().await?;
            user_credits.sort_unstable_by(|u1, u2| {
                let u1_id = u1.id.parse::<i32>().unwrap_or_default();
                let u2_id = u2.id.parse::<i32>().unwrap_or_default();
                u2_id.cmp(&u1_id)
            });
            let newest_id = user_credits[0].id.parse::<i32>().unwrap_or_default() + 1;

            let request_data = UserCredit {
                id: newest_id.to_string(),
                username: user_name.to_string(),
                user_id: user_id.to_string(),
                credits: 100,
            };

            let response = HTTP_CLIENT
                .post(format!("{}{}", server_endpoint, "/credit"))
                .json(&request_data)
                .bearer_auth(AUTHENTICATION.read().await.token.clone())
                .send()
                .await?;

            match response.error_for_status() {
                Ok(res) => {
                    if res.status() == StatusCode::CREATED {
                        Ok(res.json().await?)
                    } else {
                        Err(anyhow::anyhow!(
                            "An unknown error occurred when creating credit user: {}",
                            res.text().await?
                        ))
                    }
                }
                Err(e) => Err(anyhow::anyhow!("Error when creating credit user: {}", e)),
            }
        }
        Err(e) => Err(anyhow::anyhow!(
            "Error when fetching all user credits: {}",
            e
        )),
    }
}
