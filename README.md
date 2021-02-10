# magmsg-server

A server that stores and backs up the messages sent to the edge

## Environment variable configuration

This application requires certain environment variables to be present before it will start.

```
PORT=<the port to run at>
USER_ACCESS_TOKEN=<some kind of string token for bearer auth>
AUTH_HEADER_KEY=<HTTP request header key to check auth token from>
AWS_ACCESS_KEY_ID=<aws access id>
AWS_SECRET_ACCESS_KEY=<aws secret access key>
AWS_DYNAMO_DB_TABLE_NAME=<the name of the table>
AWS_ASSUME_ROLE_ARN=<a role to assume to get permissions to table>
LOGGING_ASSUME_ROLE_ARN=<a role to assume to get permissions to log>
AWS_REGION=<aws region>
```

## Resources

- [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [Rocket guide] (https://rocket.rs/v0.4/guide/introduction/)
- [Rusoto - AWS SDK for Rust](https://github.com/rusoto/rusoto)
- [Rusoto docs](https://www.rusoto.org/index.html)
- [Dynomite](https://github.com/softprops/dynomite)
- [Rusoto Auth example](https://github.com/lucdew/rusoto-example)
- [Cool blog about error handling](https://nick.groenen.me/posts/rust-error-handling/)
