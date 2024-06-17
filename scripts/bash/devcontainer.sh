#!/bin/bash

# Generate secrets
RPC_SECRET=$(openssl rand -hex 32)
ADMIN_TOKEN=$(openssl rand -base64 32)
METRICS_TOKEN=$(openssl rand -base64 32)

# Create garage.toml file
cat >garage.toml <<EOF
metadata_dir = "/tmp/meta"
data_dir = "/tmp/data"
db_engine = "sqlite"

replication_factor = 1

rpc_bind_addr = "[::]:3901"
rpc_public_addr = "127.0.0.1:3901"
rpc_secret = "${RPC_SECRET}"

[s3_api]
s3_region = "garage"
api_bind_addr = "[::]:3900"
root_domain = ".s3.garage.localhost"

[s3_web]
bind_addr = "[::]:3902"
root_domain = ".web.garage.localhost"
index = "index.html"

[k2v_api]
api_bind_addr = "[::]:3904"

[admin]
api_bind_addr = "[::]:3903"
admin_token = "${ADMIN_TOKEN}"
metrics_token = "${METRICS_TOKEN}"
EOF

# Activate the virtual environment
source /opt/venv/bin/activate

# Run maturin develop only if in a dev container
if [[ ${IS_DEV_CONTAINER} == "true" ]]; then
	maturin develop -m python_wrapper/Cargo.toml
fi
