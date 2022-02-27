# Hooksaurus Auctions

This is an open-source web application for running auctions to benefit non-profits. It has been built as an extension for the site [hooksaurus.com](https://hooksaurus.com) in order to support various non-profits via fundraising activities.

## Contributing

This project uses Rust 1.59 or later. To contribute you will need rust tooling. After you set up your local environment, you can run this project locally with `cargo run`.

### Setting Up Your Local Environment

You will need the following environment variables set in order to work on this project:

```sh
$ cat .envrc
#!/bin/bash

export VERSION="0.1.0beta"
export DATABASE_URL="postgresql://username:password@localhost:5432/hooksaurus_auctions"
export HMAC_KEY="some-long-secret-token"
```

### Database and Migrations

This project is using `sqlx` for querying database tables and [`sqlx-cli`](https://github.com/launchbadge/sqlx/blob/6e1c7a999a514be2df809f36f26bd5758b96c448/sqlx-cli/README.md#enable-building-in-offline-mode-with-query) for running migrations. To work with `sqlx-cli`, you will need an environment variable `DATABASE_URL` in your environment, which points to your local database.

Note: due to runtime conflicts, we _do not_ include `sqlx-cli` as a dev-dependency in Cargo.toml, so it must be installed separately. To install it, run the following:

```sh
$ cargo install sqlx-cli --features postgres
    Finished release [optimized] target(s) in 4m 09s
   Replacing ~/.cargo/bin/cargo-sqlx
   Replacing ~/.cargo/bin/sqlx
    Replaced package `sqlx-cli v0.5.7` with `sqlx-cli v0.5.7` (executables `cargo-sqlx`, `sqlx`)
```

The bootstrap a local version of your database, run the following:

```sh
$ sqlx database create
# No output
```

To create a new _reversible_ migration, run the following:

```sh
$ sqlx migrate add -r base_user_tables
Creating migrations/20211001154420_base_user_tables.up.sql
Creating migrations/20211001154420_base_user_tables.down.sql

Congratulations on creating your first migration!
```

To run all migrations, run the following:

```sh
$ sqlx migrate run
Applied 20211001154420/migrate base user tables (30.934652ms)
```

You now can see available applied migrations with:

```sh
$ sqlx migrate info
20211001154420/installed base user tables
20211001154420/installed base user tables
```

You can _revert_ the latest migration like this:

```sh
$ sqlx migrate revert
Applied 20211001154420/revert base user tables (20.160145ms)
```

#### Test Development

This application relies on a fake server from wiremock. Wiremock spins up a web server on an arbitrary port on `localhost` and so our application code can issue _real_ HTTP requests to this mock server.

Here's an example of mock-server created, started, and a new endpoint registered, which can be requested by client code:

```rust
    let mock_server = MockServer::start().await;
    let address = format!("http://{}", mock_server.address().to_string());
    let sample = tokio::fs::read("./src/tests/fixtures/sample_data.json")
        .await
        .map(|data| serde_json::from_slice::<some_data::Data>(&data[..]).unwrap())
        .map_err(|e| {
            eprintln!("IO error: {:?}", e);
        })
        .unwrap();

    let mock = Mock::given(method("GET"))
        .and(path(format!("{}", SomeEndpoint::SomeData(1))))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(serde_json::to_string(&deal).unwrap()),
        );
    mock_server.register(mock).await;
```

This endpoint returns a JSON-serialized data-structure, which was loaded from a file.
