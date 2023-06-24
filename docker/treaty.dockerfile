FROM rust as build
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler libprotobuf-dev openssl libssl-dev

# Create a new empty shell project
RUN USER=root cargo new --bin treaty
WORKDIR /treaty
RUN useradd -ms /bin/bash treaty

COPY ./treaty/Cargo.lock ./Cargo.lock
COPY ./treaty/Cargo.toml ./Cargo.toml
COPY ./treaty/src ./src
COPY ./treaty/proto ./proto
COPY ./treaty/Settings.toml ./Settings.toml
COPY ./treaty/treaty-types ./treaty-types
COPY ./treaty/treaty-http-endpoints ./treaty-http-endpoints
COPY ./treaty/treaty-client ./treaty-client 
COPY ./treaty/treaty-client-wasm ./treaty-client-wasm 
COPY ./treaty/treaty-proxy ./treaty-proxy 
COPY ./treaty/treaty-tests ./treaty-tests

# Build for release.
RUN cargo build --release

# The final base image
FROM debian:oldstable-slim
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
    openssl libglib2.0-dev libgtk2.0-dev libssl-dev

# Copy from the previous build
RUN mkdir /usr/src/treaty
WORKDIR /usr/src/treaty
RUN useradd -ms /bin/bash treaty
COPY --from=build /treaty/target/release/treaty /usr/src/treaty/treaty
COPY --from=build /treaty/Settings.toml /usr/src/treaty/Settings.toml
RUN chown -R treaty:treaty /usr/src/treaty
RUN chmod 755 /usr/src/treaty
USER treaty

EXPOSE 50051/tcp
EXPOSE 50052/tcp
EXPOSE 50055/tcp

# Run the binary
CMD ["/usr/src/treaty/treaty"]

# To build, run this from the root of the repo
# docker build -t treaty -f docker/treaty.dockerfile .