#!/bin/bash

function is_installed() {
    pip show "$1" &> /dev/null
    return $?
}

# Check if maturin is installed.
if ! command -v maturin &> /dev/null; then
    echo "Maturin is not installed. Please install maturin first."
    exit 1
fi

# Check if the module python_wrapper exists.
if ! is_installed "python_wrapper"; then
    echo "Module python_wrapper not found. Running maturin develop..."
    maturin develop -m python_wrapper/Cargo.toml
else
    echo "Module python_wrapper is already installed."
fi
