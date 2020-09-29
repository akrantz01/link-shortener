FROM ekidd/rust-musl-builder:stable AS builder

# Setup cargo
RUN USER=root cargo new --bin link-shortener
WORKDIR ./link-shortener

# Build the dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# Copy source
ADD --chown=rust:rust . ./

# Build the project
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/link_shortener*
RUN cargo build --release

FROM alpine:latest

# Configure app
ARG APP=/usr/src/app
EXPOSE 3030
ENV TZ=Etc/UTC \
    APP_USER=links \
    RUST_LOG=info \
    ADDRESS=0.0.0.0:3030

# Create the user
RUN addgroup -S $APP_USER && \
    adduser -S -g $APP_USER $APP_USER

# Install certificates and timezone stuff
RUN apk update && \
    apk add --no-cache ca-certificates tzdata && \
    rm -rf /var/cache/apk/*

# Copy the built binary
COPY --from=builder /home/rust/src/link-shortener/target/x86_64-unknown-linux-musl/release/link-shortener ${APP}/link-shortener
RUN chown -R $APP_USER:$APP_USER ${APP}

# Final configuration
USER $APP_USER
WORKDIR ${APP}
CMD ["./link-shortener"]
