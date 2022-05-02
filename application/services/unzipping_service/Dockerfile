FROM ekidd/rust-musl-builder:latest as rust-build

RUN echo "fn main() {}" > dummy.rs

COPY --chown=rust:rust Cargo.lock Cargo.lock
COPY --chown=rust:rust Cargo.toml Cargo.toml

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release 
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY --chown=rust:rust src   src
COPY --chown=rust:rust Config.toml  Config.toml

RUN cargo build --release

FROM scratch

COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/unzipping_service .
COPY --from=rust-build /home/rust/src/Config.toml .

EXPOSE 8002

CMD ["./unzipping_service"]