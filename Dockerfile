ARG RUST_VERSION=1.83.0

FROM rust:${RUST_VERSION}-slim-bookworm AS builder
WORKDIR /app
COPY . .
RUN \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release && \
    cp ./target/release/reminders_server /

FROM debian:bookworm-slim AS final
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "10001" \
    appuser
COPY --from=builder /reminders /usr/local/bin
RUN chown appuser /usr/local/bin/reminders
COPY --from=builder /app/config /opt/reminders/config
RUN chown -R appuser /opt/reminders
USER appuser
ENV RUST_LOG="reminders=debug,info"
WORKDIR /opt/reminders
ENTRYPOINT ["reminders"]
EXPOSE 8080/tcp
