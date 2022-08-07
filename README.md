# Notes
<img src="https://github.com/gymnasy55/notes/actions/workflows/ci.yml/badge.svg" alt="ci_status" />

My test Rust project.


## Prerequisites

* Install **Rust**:

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

* Add cargo env to your `.*rc` file:

```shell
source "$HOME/.cargo/env"
```

* Install **diesel_cli**:

```shell
cargo install diesel_cli --no-default-features --features postgres
```

* Create `.env` file and write `DATABASE_URL` variable:

```shell
echo "DATABASE_URL=postgres://<username?>:<password?>@localhost:<port?>/<database_name>" > .env
```

* Run migrations:

```shell
diesel migration run
```


## Run

* Build

```shell
cargo build
```

* Run

```shell
cargo run -p backend
```


## Test

* Build

```shell
cargo build
```

* Test

```shell
cargo test
```
