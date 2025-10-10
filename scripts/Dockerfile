FROM rust:1.86-bullseye AS builder

# build-time args
ARG BACKEND_DIR
ARG APP_NAME
ARG APP_PORT=3000

# install build deps
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang libpq-dev pkg-config

WORKDIR /app

# copy source
COPY $BACKEND_DIR/Cargo.toml ./$BACKEND_DIR/Cargo.toml
COPY $BACKEND_DIR/Cargo.lock ./$BACKEND_DIR/Cargo.lock
COPY $BACKEND_DIR/src ./$BACKEND_DIR/src
COPY $BACKEND_DIR/static ./$BACKEND_DIR/static
COPY $BACKEND_DIR/.sqlx ./$BACKEND_DIR/.sqlx
COPY $BACKEND_DIR/database/migrations ./$BACKEND_DIR/database/migrations

# build release
ENV SQLX_OFFLINE=true
RUN cd $BACKEND_DIR && cargo build --release

# runtime image
FROM debian:bookworm-slim AS runtime

# re-declare args (so runtime container sees them)
ARG BACKEND_DIR
ARG APP_NAME
ARG APP_PORT=3000
ENV APP_NAME=${APP_NAME}
ENV APP_PORT=${APP_PORT}

WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates libpq5 \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# copy binary
COPY --from=builder /app/$BACKEND_DIR/target/release/$APP_NAME /app/$APP_NAME
RUN chmod +x /app/$APP_NAME

# copy resources
COPY --from=builder /app/$BACKEND_DIR/Cargo.toml /app/
COPY --from=builder /app/$BACKEND_DIR/static /app/static

EXPOSE $APP_PORT

CMD ["/app/english-coaching"]
