use std::sync::Arc;

use rusoto_core::Region;
use rusoto_credential::StaticProvider;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_sts::StsClient;

pub fn get_creds(
    role_arn: String,
    external_id: String,
    region: Region,
    user_creds_provider: Option<StaticProvider>,
) {
    let credentials_provider = match (user_creds_provider) {
        Some(cp) => cp,
        None => crate::appenv::assume_role_user_creds(),
    };
    let http_client = rusoto_core::HttpClient::new().unwrap();
    let arced_client = Arc::new(http_client);
    let client = StsClient::new_with(arced_client, credentials_provider, region);
}

pub fn get_dynamo_client(
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
    region: Region,
) {
    let client = DynamoDbClient::new(region);
}
