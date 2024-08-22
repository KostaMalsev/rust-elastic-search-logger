# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:1.70 as build
ARG TARGETARCH

RUN echo "export PATH=/usr/local/cargo/bin:$PATH" >> /etc/profile
WORKDIR /app

COPY ["./platform.sh", "./"]
RUN ./platform.sh
COPY ["./config", ".cargo/config"]
RUN rustup target add $(cat /.platform)
RUN apt-get update && apt-get install -y $(cat /.compiler)

COPY ["./elasticlog/Cargo.toml", "./elasticlog/Cargo.lock", "./"]

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release --target=$(cat /.platform)

COPY ["./elasticlog/src", "./src"]

RUN touch src/main.rs && cargo build --release --target=$(cat /.platform)

RUN mkdir -p /release/$TARGETARCH
RUN cp ./target/$(cat /.platform)/release/elasticlog /release/$TARGETARCH/elasticlog

FROM gcr.io/distroless/cc-debian11
ARG TARGETARCH

COPY --from=build /release/$TARGETARCH/elasticlog /usr/local/bin/elasticlog

# Set the command to run the elastic binary
CMD ["/usr/local/bin/elasticlog"]