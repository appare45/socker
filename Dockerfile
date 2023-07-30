FROM rust:latest


COPY ./test-container /app
COPY ./app /app
WORKDIR /app
RUN cargo install --path .
