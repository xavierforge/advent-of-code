# === Stage 1: Planner (Compute dependency recipe) ===
FROM rust:1.89-slim-bookworm as planner
WORKDIR /app
# Install cargo-chef
RUN cargo install cargo-chef
COPY . .
# Generate recipe.json (contains fingerprints of all dependencies)
RUN cargo chef prepare --recipe-path recipe.json

# === Stage 2: Cacher (Compile dependencies only, not your code) ===
FROM rust:1.89-slim-bookworm as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
# Key step: This downloads and compiles all crates (tokio, nom, etc.)
# As long as Cargo.toml remains unchanged, this layer is cached by Docker and won't be rerun!
RUN cargo chef cook --release --recipe-path recipe.json

# === Stage 3: Builder (Compile your code) ===
FROM rust:1.89-slim-bookworm as builder
WORKDIR /app
# Copy the compiled dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
# Copy your source code
COPY . .
# Build the entire workspace (including all days and common modules)
# Since dependencies are already compiled, this step will be extremely fast
RUN cargo build --release --workspace

# === Stage 4: Runtime (Minimal execution environment) ===
FROM debian:bookworm-slim
WORKDIR /app

# Install necessary system libraries (some crates require openssl or ca-certificates)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# [Key Modification]
# Copy all executables starting with "day-" and containing "part" (e.g., day-01-part1)
# Use wildcards to ensure future days are automatically included
COPY --from=builder /app/target/release/day-*-part* /usr/local/bin/

# Set PATH to ensure direct execution
ENV PATH="/usr/local/bin:${PATH}"

# Default command
CMD ["echo", "Container ready! Usage: docker run --rm aoc-2025 day-01-part1"]