# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/engine/reference/builder/

################################################################################
# Create a stage for building the application.

# ARG RUST_VERSION=1.77
# ARG APP_NAME=petanque-frontend
# FROM rust:${RUST_VERSION}-slim-bullseye AS build
# ARG APP_NAME
# WORKDIR /app

# RUN rustup target add wasm32-unknown-unknown
# RUN cargo install trunk wasm-bindgen-cli

# # Build the application.
# # Leverage a cache mount to /usr/local/cargo/registry/
# # for downloaded dependencies and a cache mount to /app/target/ for 
# # compiled dependencies which will speed up subsequent builds.
# # Leverage a bind mount to the src directory to avoid having to copy the
# # source code into the container. Once built, copy the executable to an
# # output directory before the cache mounted /app/target is unmounted.
# RUN --mount=type=bind,source=./frontend/src,target=src \
#     --mount=type=bind,source=./frontend/Cargo.toml,target=Cargo.toml \
#     --mount=type=bind,source=./frontend/Cargo.lock,target=Cargo.lock \
#     --mount=type=bind,source=./frontend/index.html,target=index.html \
#     --mount=type=bind,source=./frontend/style.scss,target=style.scss \
#     --mount=type=bind,source=.env,target=.env \
#     --mount=type=cache,target=/app/target/ \
#     --mount=type=cache,target=/usr/local/cargo/registry/ \
#     <<EOF
# set -e
# trunk build --release
# cp -r ./dist/ /bin/dist/
# EOF

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the debian bullseye image as the foundation for running the app.
# By specifying the "bullseye-slim" tag, it will also use whatever happens to be the
# most recent version of that tag when you build your Dockerfile. If
# reproducability is important, consider using a digest
# (e.g., debian@sha256:ac707220fbd7b67fc19b112cee8170b41a9e97f703f588b2cdbbcdcecdd8af57).
FROM httpd:alpine AS final

# Copy the executable from the "build" stage.
# COPY --from=build /bin/dist/. /usr/local/apache2/htdocs/.
COPY ./frontend/dist/. /usr/local/apache2/htdocs/.

# Expose the port that the application listens on.
EXPOSE 80