FROM rust as build
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler libprotobuf-dev openssl

USER root
RUN useradd -ms /bin/bash treaty
RUN mkdir /treaty
RUN mkdir /treaty-admin
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
RUN cd /treaty-admin
WORKDIR /treaty-admin
COPY ./treaty-admin/Cargo.toml ./Cargo.toml
COPY ./treaty-admin/src ./src 
COPY ./treaty-admin/index.html ./index.html 
COPY ./treaty-admin/index.scss ./index.scss
RUN cd /treaty-admin
RUN ls /treaty
RUN ls /treaty-admin

# Build for release.
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN ls /treaty-admin
RUN cargo build --release
RUN ls /treaty-admin
RUN trunk build

RUN ls /treaty-admin

# The final base image
FROM httpd:latest
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
    openssl libglib2.0-dev libgtk2.0-dev

# Copy from the previous build
# COPY --from=build /treaty-admin/dist/ /var/www/html/
COPY --from=build /treaty-admin/dist/ /usr/local/apache2/htdocs/

# RUN ls /var/www/html
RUN ls /usr/local/apache2/htdocs/

# EXPOSE 50051/tcp
# EXPOSE 50052/tcp
# EXPOSE 50055/tcp
# EXPOSE 50040/tcp
EXPOSE 80/tcp

# To build, run this from the root of the repo
# docker build --progress=plain -t treaty-admin -f docker/treaty-admin.dockerfile .
# docker build --no-cache --progress=plain -t treaty-admin -f docker/treaty-admin.dockerfile .