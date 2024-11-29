# zero 2 prod

Rust project workthrough by Johannes Jyrgenson

## How To's

### running with trace level logs on powershell terminal
use `$env:RUST_LOG="trace"; cargo run` as the run command to get traces displayed in info logs

## Issues

### sqlx prepare is missing one or more queries
Run `cargo sqlx prepare -- --all-targets` locally, then the new queries should be added as .json into the .sqlx folder.
Push changes to git.
