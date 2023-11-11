# Globed Server

## Central

The central server doesn't use any databases and uses a JSON file for configuration. By default, the configuration file is put in `central-conf.json` in the current working directory when you run the server, but it can be overriden with the environment variable `GLOBED_CONFIG_PATH`. The path can be a folder or a full file path.

### Central server configuration

The central server allows for configuration hot reloading, so you can modify the config and see updates in real time without restarting the server. Some of the values (like IP addresses, secret key) will only apply on the next restart.

The following is a simple configuration template with descriptions of all options. Don't worry about creating it if you haven't already, as it will be created automatically if it doesn't already exist when you start the server.

```js
{
    "web_mountpoint": "/", // server HTTP mountpoint
    "web_address": "0.0.0.0:41000", // server address
    "gd_api": "http://www.boomlings.com/database/getGJComments21.php", // can be a proxy for the comments endpoint
    // security related
    "secret_key": "random", // secret key for auth codes, keep it secure and don't use the default value.
    "game_server_password": "random", // password for game servers to connect, see below
    "challenge_expiry": 60, // amount of seconds before an auth challenge expires
    "challenge_level": 1, // level ID for challenges, must be a valid non-unlisted level
    "token_expiry": 1800, // amount of seconds before a token expires
}
```

The minimum log level is `Warn` for other crates and `Trace` for the server itself in debug builds. In release builds, the default is `Info` and you can change it to `Warn` by defining the environment variable `GLOBED_LESS_LOG=1`

## Game

todo!

To bridge the servers together, you must set a password in the central server configuration file. After that you simply pass it as the environment variable `GLOBED_GS_CENTRAL_PASSWORD`. Once you boot up the game server it will try to make a request to the central server and tell you if the authentication succeeded or not.

The minimum log levels are the same as in the central server, except the environment variable to change them is `GLOBED_GS_LESS_LOG`