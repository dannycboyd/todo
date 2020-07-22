FROM rust:1.45 as initial_step
# install diesel and my app to /usr/local/cargo/bin
# Eventually this file will create a base image which has the build artifacts from my app, allowing faster recompiles
WORKDIR /usr/src/to_do
COPY . .
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install --path .
