#!/usr/bin/env bash

echo 'Sourcing $HOME/.cargo/env ...'

source $HOME/.cargo/env

echo 'Installing tools ...'
cargo install cargo-watch
cargo install diesel_cli --no-default-features --features sqlite


# dev environment
echo 'DATABASE_URL=file:books.db' > .env
