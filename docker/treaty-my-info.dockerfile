FROM rust as build
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler libprotobuf-dev openssl

USER root
RUN useradd -ms /bin/bash treaty
RUN mkdir /treaty
RUN mkdir /treaty-my-info
RUN cd /treaty
WORKDIR /treaty
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
RUN cd /treaty-my-info
WORKDIR /treaty-my-info
COPY ./treaty-my-info/Cargo.toml ./Cargo.toml
COPY ./treaty-my-info/src ./src 
COPY ./treaty-my-info/index.html ./index.html 
COPY ./treaty-my-info/index.scss ./index.scss
RUN cd /treaty-my-info
RUN ls /treaty
RUN ls /treaty-my-info

# Build for release.
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN ls /treaty-my-info
RUN cargo build --release
RUN ls /treaty-my-info
RUN trunk build

RUN ls /treaty-my-info

# The final base image
FROM httpd:latest
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
    openssl libglib2.0-dev libgtk2.0-dev

# Copy from the previous build
# COPY --from=build /treaty-my-info/dist/ /var/www/html/
COPY --from=build /treaty-my-info/dist/ /usr/local/apache2/htdocs/

# RUN ls /var/www/html
RUN ls /usr/local/apache2/htdocs/

# EXPOSE 50051/tcp
# EXPOSE 50052/tcp
# EXPOSE 50055/tcp
# EXPOSE 50040/tcp
EXPOSE 80/tcp

# To build, run this from the root of the repo
# docker build --progress=plain -t treaty-my-info -f docker/treaty-my-info.dockerfile .
# docker build --no-cache --progress=plain -t treaty-my-info -f docker/treaty-my-info.dockerfile .