# Play Actix Web

## Setup

Firstly, please make sure you already had MySQL up and running.

Next, setup db by running the `db_schema.sql` script. 

When `db_schema.sql` has executed successfully, run the tests:
```shell
cargo test
```

After the tests have completed and all passed, startup the application:
```shell
cargo run
```

The following message should be printed to the terminal:
```shell
=== Playing Actix Web APIs ===
Using configuration file from config.json
Connected to database: mysql://dev_guy:actIX_me1881@localhost/user_man
Listening on: 127.0.0.1:8008
```
Depending on your setup, the log on console could be different from me.

## Test Coverage

All expected paths of behavior are tested.
These test serve as an example for what is sufficient test coverage for an initial application.

P/S: not all routes are fully tested. 