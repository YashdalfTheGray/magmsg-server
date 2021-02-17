# magmsg-server

A server that stores and backs up the messages sent to the edge

## API

The API for this is fairly simple. The main primitive is a message. Operations except for the UD in CRUDL are supported currently - create, read, list. The fields in a message are,

| Name        | Type          | Description                                                   |
| ----------- | ------------- | ------------------------------------------------------------- |
| `messageId` | String (UUID) | an ID that identified that specific message                   |
| `createdAt` | Number        | the time when the message was created, in ms since Unix Epoch |
| `content`   | String        | the actual message                                            |
| `createdBy` | String        | the person who created it                                     |

## Environment variable configuration

This application requires certain environment variables to be present before it will start.

```
PORT=<the port to run at, optional, defaults to 8080>
USER_ACCESS_TOKEN=<some kind of string token for bearer auth>
AUTH_HEADER_KEY=<HTTP request header key to check auth token from>
AWS_ACCESS_KEY_ID=<aws access id>
AWS_SECRET_ACCESS_KEY=<aws secret access key>
AWS_DYNAMO_DB_TABLE_NAME=<the name of the table, optional, defaults to "messages">
AWS_ASSUME_ROLE_ARN=<a role to assume to get permissions to table>
LOG_FORMAT=<the format of the request log lines, optional, defaults to "dev">
LOGGING_ASSUME_ROLE_ARN=<a role to assume to get permissions to log>
LOGGING_BUCKET_NAME=<an S3 bucket to send the logs to, optional>
LOG_WRITE_INTERVAL=<how often to push logs to S3, in seconds, optional, defaults to 15 minutes>
APPLICATION_LOG_PATH=<where to put the application logs, optional, defaults to "logs/application.log">
AWS_REGION=<aws region>
```

## Request log Formats

| Format name | Format string                                                                            |
| ----------- | ---------------------------------------------------------------------------------------- |
| `dev`       | `{method} {uri} {status} {response_length} - {response_time}ms`                          |
| `standard`  | `{client_addr} [{date}] "{method} {uri}" {status} {response_length} - {response_time}ms` |
| `short`     | `{client_addr} {method} {uri} {status} {response_length} - {response_time}ms`            |

## Resources

- [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [Rocket guide] (https://rocket.rs/v0.4/guide/introduction/)
- [Rusoto - AWS SDK for Rust](https://github.com/rusoto/rusoto)
- [Rusoto docs](https://www.rusoto.org/index.html)
- [Dynomite](https://github.com/softprops/dynomite)
- [Rusoto Auth example](https://github.com/lucdew/rusoto-example)
- [Cool blog about error handling](https://nick.groenen.me/posts/rust-error-handling/)
