

# Use Rust stable
FROM rust:1.86-bullseye AS builder

# Use environment variables for customization
ARG BACKEND_DIR
ARG APP_NAME
ARG PORT=3000

# Install necessary tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang libpq-dev pkg-config

# Create /app directory
RUN mkdir -p /app
WORKDIR /app

# Copy Backend files
COPY $BACKEND_DIR/Cargo.toml ./$BACKEND_DIR/Cargo.toml
COPY $BACKEND_DIR/Cargo.lock ./$BACKEND_DIR/Cargo.lock

COPY $BACKEND_DIR/src ./$BACKEND_DIR/src
COPY $BACKEND_DIR/static ./$BACKEND_DIR/static

COPY $BACKEND_DIR/.sqlx ./$BACKEND_DIR/.sqlx

# Build application with SQLx offline mode
ENV SQLX_OFFLINE=true
RUN cd $BACKEND_DIR && cargo build --release

FROM debian:bookworm-slim AS runtime

# Use environment variables for customization
ARG BACKEND_DIR
ARG APP_NAME
ARG PORT=3000

WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates libpq5 \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/$BACKEND_DIR/target/release/$APP_NAME /app/$APP_NAME

# Make the binary executable
RUN chmod +x /app/$APP_NAME

# Copy Cargo.toml if it's needed at runtime
COPY --from=builder /app/$BACKEND_DIR/Cargo.toml /app/

# Copy static files if needed
COPY --from=builder /app/$BACKEND_DIR/static /app/static

EXPOSE $PORT

# Run the server
CMD ["/app/$APP_NAME"]