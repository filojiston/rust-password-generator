FROM scratch
COPY ./target/x86_64-unknown-linux-musl/release/rust-password-generator /rust-password-generator
ENTRYPOINT ["/rust-password-generator"]
