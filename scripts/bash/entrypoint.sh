#!/bin/bash

# Activate the virtual environment
source /opt/venv/bin/activate

# Check for arguments to determine the mode of operation.
if [[ "$1" == "python" ]]; then
    shift
    exec python3 "$@"
elif [[ "$1" == "rust" ]]; then
    exec ./target/release/wspr_cdk_server
else
    echo "Invalid option. Use 'python' to run the Python server or 'rust' to run the Rust server."
    exit 1
fi
