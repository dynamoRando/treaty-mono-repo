FROM rust as build
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler libprotobuf-dev openssl

# Create a new empty shell project
RUN USER=root cargo new --bin treaty-proxy
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
RUN cargo build -p treaty-proxy --release

# The final base image
FROM debian:stable-slim
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
    openssl libglib2.0-dev libgtk2.0-dev

# Copy from the previous build
RUN mkdir /usr/src/treaty-proxy
WORKDIR /usr/src/treaty-proxy
RUN useradd -ms /bin/bash treaty
COPY --from=build /treaty/target/release/treaty-proxy /usr/src/treaty-proxy/treaty-proxy
COPY --from=build /treaty/treaty-proxy/Settings.toml /usr/src/treaty-proxy/Settings.toml
RUN chown -R treaty:treaty /usr/src/treaty-proxy
RUN chmod 775 -R /usr/src/treaty-proxy
USER treaty

EXPOSE 50051/tcp
EXPOSE 50052/tcp
EXPOSE 50055/tcp
EXPOSE 50040/tcp

# Run the binary
CMD ["/usr/src/treaty-proxy/treaty-proxy"]

# To build, run this from the root of the repo
# docker build -t treaty-proxy -f docker/treaty-proxy.dockerfile .