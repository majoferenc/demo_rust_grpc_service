# Rust gRPC Service

## Generating init project

    cargo init

## Run Account Microservice

    cargo run --bin accounts-microservice

## Run Account Client

    cargo run --bin accounts-client

## Build binaries (Debug Mode)

    cargo build

### Location of binaries
    
    ./target/degug/accounts-client
    ./target/debug/accounts-microservice

## Build binaries (Release Mode)

    cargo build --release

### Location of binaries
    
    ./target/release/accounts-client
    ./target/release/accounts-microservice
