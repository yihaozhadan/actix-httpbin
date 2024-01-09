# https://www.codefeetime.com/post/docker-config-for-actix-web-diesel-and-postgres/#3-the-optimized-dockerfile-60mb-image-size
# ---------------------------------------------------
# 1 - Build Stage
#
# Use official rust image to for application build
# ---------------------------------------------------
FROM rust:latest as build

# Setup working directory
WORKDIR /usr/src/actix-httpbin
COPY . .

# Build application
RUN cargo install --path .

# ---------------------------------------------------
# 2 - Deploy Stage
#
# Use a distroless image for minimal container size
# - Copy application files into the image
# ---------------------------------------------------
FROM gcr.io/distroless/cc-debian12

# Set the architecture argument (arm64, i.e. aarch64 as default)
# For amd64, i.e. x86_64, you can append a flag when invoking the build `... --build-arg "ARCH=x86_64"`
# ARG ARCH=aarch64

# Application files
COPY --from=build /usr/local/cargo/bin/actix-httpbin /usr/local/bin/actix-httpbin

CMD ["actix-httpbin"]
