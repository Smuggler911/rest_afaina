FROM rust:1.86

WORKDIR /app

COPY Cargo.toml ./

COPY src/main.rs  /app/src/
RUN  RUSTFLAGS='-C target-cpu=native'
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/afaina_service"]