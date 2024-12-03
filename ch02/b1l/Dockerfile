FROM rust:latest

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    libudev-dev \
    libusb-1.0-0-dev \
    build-essential \
    pkg-config \
    curl \
    && apt-get clean

# Add the ARM target
RUN rustup target add thumbv7em-none-eabihf

# Install probe-rs using the recommended script
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
