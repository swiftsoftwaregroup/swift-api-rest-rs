# swift-api-rest-rs
REST Web API using Rust and Actix Web

# cli-rust

Template for Command Line Interface (CLI) tool in Rust

## Development

### Setup for macOS

Install `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Test:

```bash
source $HOME/.cargo/env
rustc --help
```

### Work on macOS

Configure project:

```bash
source configure.sh
```

Open the project in Visual Studio Code:

```bash
code .
```

Setup DB migrations:

```bash
diesel setup
```

Gneerate DB migration:

```bash
diesel migration generate create_books
```

Apply DB migrations:

```bash
diesel migration run
```

### Run

```bash
cargo run
```

### Build

```bash
cargo build
```

## How to create a new project

```bash
# create new project
cargo init

# add packages
cargo add actix-web dotenv
cargo add diesel --features sqlite,r2d2

cargo add serde --features derive
cargo add serde_json
cargo add chrono --features serde
```

Install Diesel CLI:

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

