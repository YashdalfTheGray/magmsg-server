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

### `GET /api`

Just a status check, mostly meant for canaries or pings to the system to make sure that everything is working just fine. Returns JSON and does not require authentication using HTTP headers.

### `GET /api/messages?fields`

Gets a list of all the messages, optionally with only the specified fields. The fields can be passed in as CSV and valid fields are listed above. Returns JSON and requires authentication using HTTP headers.

Examples

- `GET /api/messages` - get all messages with all the fields in them as a JSON array
- `GET /api/messages?fields=content,messageId` - get all messages just with the `content` and `messageId` fields as a JSON array.

### `GET /api/messages/<messageId>?fields`

Gets one specific message specified by the passed in message ID. Returns a 404 if message is not found, otherwise returns the message encoded as JSON. This endpoint also has similar filtering capabilities as the endpoint above. Requires authentication using HTTP headers.

Examples

- `GET /api/messages/<uuid>` - get a particular message
- `GET /api/messages/<uuid>?fields=content` - get only the `content` field of a particular message

### `PUT /api/messages`

Creates a new message on the server. This endpoint expects a body in JSON format and requires authentication using HTTP headers. On success, it returns an HTTP 201. The body expected is detailed below.

```json
{
  "message": "<the content of the message>",
  "author": "<who wrote this message>"
}
```

## Request logging

This server uses a Rocket.rs Fairing to enable logging information about requests that are coming into the server. The request logs are written to file every time a request comes along.

There are a couple of knobs and switches that you can change about this behavior using a `.env` file or environment variables but given a certain period (defaults to 60 minutes) and a certain Amazon S3 bucket name (defaults to `request-logs`), this will push the last 60 minutes of request logs up to S3. You will also need to provide a role that we can assume that will give us access to the S3 bucket.

Additionally, the logs folder contains a request log file that contains logs that haven't been written to S3 yet.

### Supported formats

| Format name | Format string                                                                            |
| ----------- | ---------------------------------------------------------------------------------------- |
| `dev`       | `{method} {uri} {status} {response_length} - {response_time}ms`                          |
| `standard`  | `{client_addr} [{date}] "{method} {uri}" {status} {response_length} - {response_time}ms` |
| `short`     | `{client_addr} {method} {uri} {status} {response_length} - {response_time}ms`            |

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

## Resources

- [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [Rocket guide](https://rocket.rs/v0.4/guide/introduction/)
- [Rusoto - AWS SDK for Rust](https://github.com/rusoto/rusoto)
- [Rusoto docs](https://www.rusoto.org/index.html)
- [Dynomite](https://github.com/softprops/dynomite)
- [Rusoto Auth example](https://github.com/lucdew/rusoto-example)
- [Cool blog about error handling](https://nick.groenen.me/posts/rust-error-handling/)
