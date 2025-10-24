FROM rust:1.86-bullseye AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
ARG BACKEND_DIR
COPY $BACKEND_DIR/Cargo.toml $BACKEND_DIR/Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
ARG BACKEND_DIR
ARG APP_NAME
ARG APP_PORT=3000

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang libpq-dev pkg-config \
  && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY $BACKEND_DIR/Cargo.toml $BACKEND_DIR/Cargo.lock ./
COPY $BACKEND_DIR/src ./src
COPY $BACKEND_DIR/static ./static
COPY $BACKEND_DIR/.sqlx ./.sqlx
COPY $BACKEND_DIR/database/migrations ./database/migrations

ENV SQLX_OFFLINE=true
RUN cargo build --release --bin $APP_NAME

FROM debian:bookworm-slim AS runtime

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

COPY --from=builder /app/target/release/$APP_NAME /app/$APP_NAME
RUN chmod +x /app/$APP_NAME

COPY --from=builder /app/Cargo.toml /app/
COPY --from=builder /app/static /app/static

EXPOSE $APP_PORT

CMD /app/${APP_NAME}
