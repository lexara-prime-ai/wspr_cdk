#!/bin/bash

function command_exists() {
    command -v "$1" &> /dev/null
    return $?
}

# Check if Rust is already installed.
if command_exists rustc; then
    echo "Rust is already installed."
else
    echo "Rust is not installed. Installing Rust..."

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

    . "$HOME/.cargo/env"

    echo "Rust installation completed."
fi
