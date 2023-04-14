use crate::shared::structs::google::GoogleCredential;
use google_drive::traits::FileOps;
use google_drive::Client;
use once_cell::sync::{Lazy, OnceCell};
use std::sync::{Arc, Mutex};

static GOOGLE_DRIVE_CLIENT: OnceCell<Arc<Mutex<Client>>> = OnceCell::new();

static GOOGLE_CREDENTIAL: Lazy<GoogleCredential> = Lazy::new(|| {
    let bytes = std::fs::read("google_credentials.json").unwrap_or_default();
    serde_json::from_slice(&bytes).unwrap_or_default()
});

const SCOPES: [&str; 10] = [
    "https://www.googleapis.com/auth/activity",
    "https://www.googleapis.com/auth/drive.file",
    "https://www.googleapis.com/auth/docs",
    "https://www.googleapis.com/auth/drive",
    "https://www.googleapis.com/auth/drive.activity",
    "https://www.googleapis.com/auth/drive.activity.readonly",
    "https://www.googleapis.com/auth/drive.readonly",
    "https://www.googleapis.com/auth/drive.metadata",
    "https://www.googleapis.com/auth/drive.metadata.readonly",
    "https://www.googleapis.com/auth/drive.photos.readonly",
];

pub async fn do_test() -> anyhow::Result<()> {
    let mut client = Client::new(
        &GOOGLE_CREDENTIAL.installed.client_id,
        &GOOGLE_CREDENTIAL.installed.client_secret,
        GOOGLE_CREDENTIAL.installed.redirect_uris[0].clone(),
        "ya29.a0Aa4xrXN-hset0Dr0itg9FrXpaff50cVXtI0O_F2t-HooVA7B0nBQwHFqWmthaBgjUqy0nPGVimdv4wTgpAnoTfhtp1LGx_olOV7rKXRf3qjltJ4F_5BZ5ZS9rqMZ7nJuCO8Jz1tG-HXany3zH9hJ9p9mZ8WZaCgYKATASARESFQEjDvL9IGujmowWZkEq0ph0TCs6Qw0163",
        "1//0eqj0LkbLqwynCgYIARAAGA4SNwF-L9IrSKoMjLghEJNlrj1jJzU8TKRlkfHqmlc4DD1ALC9yvJD-r9zICoL7oZq8CQ-JPzPqvKI",
    );
    client.set_auto_access_token_refresh(true);
    let client = GOOGLE_DRIVE_CLIENT.get_or_init(|| Arc::new(Mutex::new(client)));
    let drive_id = "10j4hr2wVFLUFM4MNo1k-QqsyMG3HbT0R";

    if let Ok(mut client) = client.lock() {
        /*if let Ok(result) = client.refresh_access_token().await {
            log::info!("{:?}", &result);
        }*/
        //let scopes = SCOPES.iter().map(ToString::to_string).collect::<Vec<_>>();
        //let scopes = vec![];
        //let url = client.user_consent_url(scopes.as_slice());
        //log::info!("Go to {} to authorize the Kou bot!", &url);
        let result = client
            .files()
            .list(
                "",
                "",
                true,
                "",
                true,
                "",
                100,
                "",
                "name = 'Midjourney'",
                "",
                true,
                true,
                "",
            )
            .await;
        tracing::info!("Result: {:?}", &result);
    }
    Ok(())
}
