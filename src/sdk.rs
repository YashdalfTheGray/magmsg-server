use rusoto_dynamodb::DynamoDbClient;
use rusoto_sts::StsClient;

pub fn get_creds(
    access_key_id: String,
    secret_access_key: String,
    role_arn: String,
    external_id: String,
    region: String,
) {
    let client = StsClient::new(region);
}

pub fn get_dynamo_client(
    access_key_id: String,
    secret_access_key: String,
    session_token: String,
    region: String,
) {
    let client = DynamoDbClient::new(Region::UsEast1);
}
