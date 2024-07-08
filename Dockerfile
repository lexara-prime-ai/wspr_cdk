FROM rust:latest AS chef

WORKDIR /wspr_cdk

RUN cargo install cargo-chef --locked

#----------------------------------------

# Use the chef stage to prepare the build plan.
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

#-------------------------------------------

# Use the chef stage as the base for the builder stage.
FROM chef AS builder

# Install python 3.11 development files and virtual environment tools
RUN apt-get update --no-install-recommends && \
    apt-get install libclang-dev -y && \
    apt-get install -y python3-dev python3-venv --no-install-recommends && \
    apt-get clean --no-install-recommends


# Copy the prepared recipe.json from the [planner] stage.
COPY --from=planner /wspr_cdk/recipe.json recipe.json

# Cook the dependencies according to the recipe.
RUN cargo chef cook --release --recipe-path recipe.json

# Copy [python_deps] script to manage dependencies.
# The script currently installs the following dependencies:
    # "mkdocs" "maturin" "tableauhyperapi" "google-api-python-client" "google-auth-httplib2" "google-auth-oauthlib"

COPY scripts/bash/python_deps.sh /scripts/bash/python_deps.sh

# Create and activate a virtual environment.
RUN python3 -m venv /opt/venv
ENV PATH="/opt/venv/bin:$PATH"
ENV VIRTUAL_ENV=/opt/venv

# Modify script permissions.
RUN chmod +x /scripts/bash/python_deps.sh
RUN /scripts/bash/python_deps.sh

COPY . .

# Set environment variables for Rocket
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Build the project
RUN cargo build --workspace --release

COPY /scripts/bash/entrypoint.sh /scripts/bash/entrypoint.sh
RUN chmod +x /scripts/bash/entrypoint.sh

COPY /scripts/bash/devcontainer.sh /scripts/bash/devcontainer.sh
RUN chmod +x /scripts/bash/devcontainer.sh

COPY /scripts/bash/build_wheel.sh /scripts/bash/build_wheel.sh
RUN chmod +x /scripts/bash/build_wheel.sh

# Build <python_wrapper> wheel
RUN /scripts/bash/build_wheel.sh

#------------------------------------

# Download <garage> -> ARCH: linux/amd64
RUN wget --progress=dot:giga https://garagehq.deuxfleurs.fr/_releases/v1.0.0/x86_64-unknown-linux-musl/garage

#------------------------------------


# [CURRENT] entry point.
# Run maturin develop on container initialization.
ENTRYPOINT ["/scripts/bash/entrypoint.sh"]

#------------------------------------

EXPOSE 3900-3904 8000

# NOTES #
# End users need to provide/generate their own <service_account.json> credentials.
# You can then start the container with the <credentials> mounted
# sudo docker run -it -p 8000:8000 \
#   -e GOOGLE_APPLICATION_CREDENTIALS=/service_account.json \
#   -v ./service_account.json:/wspr_cdk/service_account.json \
#   test python ./hyper/hyper/server.py --interval 10


# Building & Running [standalone] modules:
#
# cargo build --release --workspace -> Builds all <workspace> modules.
# cargo build --release -p windows_container
# cargo build --release -p wspr_cdk_server
# cargo build --release -p wspr_cdk

# cargo run --release --manifest-path ./wspr_cdk_server/Cargo.toml
# cargo run --release --manifest-path wspr_cdk/Cargo.toml
# cargo run --release --manifest-path windows_container/Cargo.toml


# Specify a range of ports to expose in the dockerfile and when running it:

#     on dockerfile:
#     EXPOSE 8888-8898
#     Build image:
#     docker build -t <image_name>:<version> -f dockerfile .
#     When running the image:
#     docker run -it -p 8888-8898:8888-8898 -v C:\x\x\x:/app <image_name>:<version>