FROM ekidd/rust-musl-builder:latest as rust-build


# Install the necessary dependencies, mostly to statically link rdkafka to rust-rdkafka and force the latest version of cmake
USER root
RUN apt remove --purge --auto-remove cmake -y
RUN sudo apt update && \
    sudo apt install -y apt-transport-https ca-certificates gnupg software-properties-common wget && \
    sudo apt clean all

RUN wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | sudo apt-key add -

RUN apt-add-repository 'deb https://apt.kitware.com/ubuntu/ bionic main'

RUN apt-get update && apt-get install -y build-essential \
    curl \
    openssl libssl-dev \
    libsasl2-dev\
    pkg-config \
    python \
    valgrind \
    zlib1g-dev \
    cmake



USER rust


# we're buildin an 'unchanging' project out of just the cargo files, to avoid rebuilding from scratch for only your changes



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

COPY --from=rust-build /home/rust/src/target/x86_64-unknown-linux-musl/release/extraction_service .
COPY --from=rust-build /home/rust/src/Config.toml .

EXPOSE 8000

CMD ["./extraction_service"]