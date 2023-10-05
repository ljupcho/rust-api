# Use a minimal Rust image as the base image
# FROM rust:latest as builder
FROM rust:bookworm as builder

# Create a new directory for your Rust application
WORKDIR /app

RUN mkdir src
RUN mkdir entity
RUN mkdir entity/src
# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./
COPY entity/Cargo.toml ./entity/

# Fetch the dependencies without building the application
RUN cargo fetch

# Copy the rest of your application source code to the container
COPY ./src/ ./src/
COPY ./entity/src/ ./entity/src/

# Build your Rust application
RUN cargo build --release

# Start a new stage for the final image
# FROM debian:buster-slim
# FROM alpine:latest
# FROM debian:bullseye-slim
FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

WORKDIR /app
# Set environment variables, if needed
# ENV MY_ENV_VAR=value
USER root
# Copy the compiled binary from the builder stage to the final image
# name rustapi is what is define under [[bin]] in Cargo.toml
COPY --from=builder /app/target/release/rustapi /app/rustapi

# Expose any necessary ports
EXPOSE 9900 

# Define the command to run your Rust application
CMD ["/app/rustapi"]
