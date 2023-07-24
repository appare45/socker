FROM debian:stable-slim 


COPY ./test-container /test-container
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /app