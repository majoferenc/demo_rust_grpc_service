FROM rust:1.65.0-slim as build

COPY ./ ./

RUN cargo build --release

FROM scratch

COPY --from=build ./target/release/accounts-microservice .

USER 1001

CMD ["./accounts-microservice"]
