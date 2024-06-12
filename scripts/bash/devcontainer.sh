#!/bin/bash

# Activate the virtual environment
source /opt/venv/bin/activate

# Run maturin develop only if in a dev container
if [[ ${IS_DEV_CONTAINER} == "true" ]]; then
	maturin develop -m python_wrapper/Cargo.toml
fi
