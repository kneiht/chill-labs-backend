# Sử dụng Rust stable
FROM rust:1.86-bullseye as builder

# Cài đặt các công cụ cần thiết
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang libpq-dev pkg-config

# Tạo thư mục /app
RUN mkdir -p /app
WORKDIR /app

# Copy Backend files
COPY backend/Cargo.toml ./backend/Cargo.toml
COPY backend/Cargo.lock ./backend/Cargo.lock

COPY backend/src ./backend/src
COPY backend/static ./backend/static

COPY backend/.sqlx ./backend/.sqlx

# Build ứng dụng với SQLx offline mode
ENV SQLX_OFFLINE=true
RUN cd backend && cargo build --release

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates libpq5 \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary to the /app directory
COPY --from=builder /app/backend/target/release/backend /app/english-coaching

# Make the binary executable
RUN chmod +x /app/english-coaching

# Copy Cargo.toml if it's needed at runtime
COPY --from=builder /app/backend/Cargo.toml /app/

# Copy các file tĩnh nếu cần
COPY --from=builder /app/backend/static /app/static

EXPOSE 3000

# Run the server
CMD ["/app/english-coaching"]