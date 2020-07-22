FROM rust:1.45 as initial_step
# install diesel and my app to /usr/local/cargo/bin
WORKDIR /usr/src/to_do
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .
