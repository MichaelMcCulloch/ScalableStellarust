# Use this custom image
FROM ekidd/rust-musl-builder:latest as rust-build

# Add the source code (+fix file permissions) 
ADD --chown=rust:rust src src
ADD --chown=rust:rust Cargo.toml Cargo.toml
ADD --chown=rust:rust Config.toml Config.toml

# Build
RUN cargo build --release
FROM scratch

# Copy the binary to a minimal Linux OS
COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/unzipping_service .
COPY --from=rust-build /home/rust/src/Config.toml .

EXPOSE 8002

CMD ["./unzipping_service"]