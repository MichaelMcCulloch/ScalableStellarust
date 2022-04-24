# Use this custom image
FROM ekidd/rust-musl-builder:latest as rust-build


# Add the source code (+fix file permissions) 
ADD --chown=rust:rust Cargo.toml  Cargo.toml
ADD --chown=rust:rust Cargo.lock  Cargo.lock

RUN echo "fn main() {}" > dummy.rs
# If this changed likely the Cargo.toml changed so lets trigger the
# recopying of it anyways
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
# We'll get to what this substitution is for but replace main.rs with
# lib.rs if this is a library
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release 
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

ADD --chown=rust:rust src   src
ADD --chown=rust:rust Config.toml  Config.toml


# Build
RUN cargo build --release

FROM scratch

# Copy the binary to a minimal Linux OS
COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/parsing_service .
# COPY --from=rust-build /home/rust/src/Config.toml .

EXPOSE 8001

CMD ["./parsing_service"]