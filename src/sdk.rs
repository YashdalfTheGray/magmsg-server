use rusoto_sts::StsClient;

pub fn get_creds(
    access_key_id: string,
    secret_access_key: string,
    role_arn: string,
    external_id: string,
    region: string,
) {
    let client = StsClient::new(region);
}
