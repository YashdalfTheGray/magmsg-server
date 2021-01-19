use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use chrono::prelude::*;
use rusoto_core::Region;
use rusoto_credential::{AwsCredentials, CredentialsError, ProvideAwsCredentials, StaticProvider};
use rusoto_dynamodb::DynamoDbClient;
use rusoto_sts::{AssumeRoleRequest, Sts, StsClient};

pub struct StsProvider {
    pub user_creds: StaticProvider,
    pub assume_role_arn: String,
    pub external_id: Option<String>,
    pub region: Region,
}

#[async_trait]
impl ProvideAwsCredentials for StsProvider {
    async fn credentials(&self) -> Result<AwsCredentials, CredentialsError> {
        let assume_role_creds_provider = crate::appenv::assume_role_user_creds();
        let region = crate::appenv::region();

        let http_client = rusoto_core::HttpClient::new().unwrap();
        let arced_client = Arc::new(http_client);
        let client = StsClient::new_with(arced_client, assume_role_creds_provider, region);

        let request: AssumeRoleRequest = AssumeRoleRequest {
            external_id: Some(crate::appenv::external_id()),
            role_arn: crate::appenv::assume_role_arn(),
            role_session_name: format!("messages-session-{}", get_time_in_millis()),
            ..Default::default()
        };

        let response = client
            .assume_role(request)
            .await
            .or_else(|err| Err(CredentialsError::new(err.to_string())))?;
        let creds = response
            .credentials
            .ok_or_else(|| CredentialsError::new("Did not find credentials in response"))?;

        Ok(AwsCredentials::new(
            creds.access_key_id,
            creds.secret_access_key,
            Some(creds.session_token),
            Some(parse_into_utc(creds.expiration)),
        ))
    }
}

pub async fn get_creds(
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

fn parse_into_utc(expiration_str: String) -> DateTime<Utc> {
    DateTime::from_utc(
        DateTime::parse_from_rfc3339(&expiration_str)
            .unwrap()
            .naive_utc(),
        Utc,
    )
}
