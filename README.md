# Rust App In Docker
This repo is a simple example of how to package a Rust app in 
a Docker container and support ENV variables (like 12-factor app)

# Requirements
 * Docker environment and familiarity with command lines

# Setup
 * Clone this repo and `cd tutorial-rust-docker`

# Build
`docker build -t my-rust-app .`

# Run (without ENV)
`docker run --rm --name rusty my-rust-app`
 * prints `Hello World`

# Run (with ENV)
`docker run --rm -e RUST_DOCKER_NAME="Foo" --name rusty my-rust-app`
 * prints `Hello, Foo`