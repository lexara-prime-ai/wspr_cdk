FROM rust:latest AS chef

WORKDIR /wspr_cdk

RUN cargo install cargo-chef --locked

#----------------------------------------

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

#-------------------------------------------

FROM chef AS builder

# Install python 3.11 development files.
# To do -> Run scripts to install additional dependencies e.g [mkdocs], [tableauhyperapi] etc.
RUN apt-get update && \
    apt-get install -y python3.11-dev && \
    apt-get clean

COPY --from=planner /wspr_cdk/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

RUN cargo build --workspace --release

#------------------------------------

CMD [ "./target/release/wspr_cdk_server"]

#------------------------------------

EXPOSE 8000