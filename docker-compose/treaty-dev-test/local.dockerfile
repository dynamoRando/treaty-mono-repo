FROM rust:latest AS build
ENV TREATY_POSTGRES_HOST="postgres"
ENV TREATY_POSTGRES_PORT=5432
ENV TREATY_POSTGRES_UN="tester"
ENV TREATY_POSTGRES_PW="tester"
ENV TREATY_POSTGRES_TREATY_SCHEMA="treaty"
ENV TREATY_POSTGRES_DB_NAME="treaty"
ENV TREATY_POSTGRES_USE_SCHEMA=true

RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler libprotobuf-dev openssl libssl-dev build-essential clang mold nano

WORKDIR /treaty
COPY treaty/ .
# RUN pwd
# RUN ls -l
# RUN cat Cargo.toml
# RUN ls treaty-client/ -l
# RUN ls treaty-http-endpoints/ -l
# RUN ls treaty-proxy/ -l
# RUN ls treaty-tests/ -l
# RUN ls treaty-types/ -l
RUN cargo build 
RUN cargo t --no-run

WORKDIR /treaty/treaty-tests 
RUN cargo t --no-run
