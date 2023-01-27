# syntax=docker/dockerfile:1
FROM debian:stable-slim
WORKDIR /


# COMPILERS
# Python3 and some stuff
RUN apt-get update && \
    apt-get -y install python3 build-essential curl

RUN apt-get update

# C
RUN apt-get update && \
    apt-get -y install gcc mono-mcs g++

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Get nodejs
RUN curl -fsSL https://deb.nodesource.com/setup_19.x | bash - && \
    apt-get install -y nodejs

# scripts
COPY to_copy /