use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_sts::StsClient;

pub fn get_creds(
    access_key_id: String,
    secret_access_key: String,
    role_arn: String,
    external_id: String,
    region: Region,
) {
    let client = StsClient::new(region);
}

pub fn get_dynamo_client(
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
    region: Region,
) {
    let client = DynamoDbClient::new(region);
}
