FROM rust:latest

WORKDIR /wspr_cdk

RUN cargo install cargo-chef --locked

#----------------------------------------

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

#-------------------------------------------

FROM chef AS builder

COPY --from=planner /wspr_cdk/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

#------------------------------------

EXPOSE 8080