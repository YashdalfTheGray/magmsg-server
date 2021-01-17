use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::prelude::*;
use rusoto_core::Region;
use rusoto_credential::{ProvideAwsCredentials, StaticProvider};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};

pub async fn get_creds<P>(
    role_arn: String,
    external_id: String,
    region: Region,
    user_creds_provider: Option<StaticProvider>,
) -> impl ProvideAwsCredentials {
    let credentials_provider = match user_creds_provider {
        Some(cp) => cp,
        None => crate::appenv::assume_role_user_creds(),
    };
    let http_client = rusoto_core::HttpClient::new().unwrap();
    let arced_client = Arc::new(http_client);
    let client = StsClient::new_with(arced_client, credentials_provider, region);

    let request: AssumeRoleRequest = AssumeRoleRequest {
        external_id: Some(external_id),
        role_arn: role_arn,
        role_session_name: format!("messages-session-{}", get_time_in_millis()),
        ..Default::default()
    };

    let response = client.assume_role(request).await.unwrap();
    let creds = response.credentials.unwrap();

    StaticProvider::new(
        creds.access_key_id,
        creds.secret_access_key,
        Some(creds.session_token),
        Some(get_time_to_expire(creds.expiration)),
    )
}

pub fn get_dynamo_client<P>(credential_provider: P, region: Region) -> DynamoDbClient
where
    P: ProvideAwsCredentials + Sync + Send + 'static,
{
    let http_client = rusoto_core::HttpClient::new().unwrap();
    let arced_client = Arc::new(http_client);

    DynamoDbClient::new_with(arced_client, credential_provider, region)
}

fn get_time_in_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_millis()
}

fn get_time_to_expire(expiration_str: String) -> i64 {
    let expiration_time = DateTime::parse_from_rfc3339(&expiration_str).unwrap();
    let right_now = Utc::now();

    expiration_time.timestamp() - right_now.timestamp()
}
