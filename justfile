set shell := ["bash", "-c"]

default:
    @just --list

# Setup environment dependencies
setup:
    cargo install cargo-generate cargo-nextest cargo-watch just
    @echo "✅ Setup complete!"

# Generate a new daily solution from template
# Usage: just create day-01
create day:
    cargo generate --path ./years/2025/daily-template --name {{day}} --destination years/2025
    @echo "🎉 Created {{day}} in years/2025!"

# Development loop: watch, check, test, and lint on file change
# Usage: just work day-01
work day part:
    cargo watch -w years/2025/{{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}"

# Run the solution in release mode
# Usage: just run day-01 part1
run day part:
    cargo run -p {{day}} --bin {{day}}-{{part}} --release

# Check code style and quality (Linter)
# Usage: just lint day-01
lint day:
    cargo clippy -p {{day}} -- -D warnings

# Run tests
# Usage: just test day-01 (runs all tests for the day)
# Usage: just test day-01 --part 1 (filters for specific tests)
test day part="":
    RUST_LOG=debug cargo nextest run -p {{day}} {{part}} --success-output immediate

# Run Local CI (Format, Clippy, and Test for the entire workspace)
ci:
    cargo fmt --all -- --check
    cargo clippy --workspace -- -D warnings
    cargo nextest run --workspace

# Build Docker image
docker-build:
    docker build -t aoc-2025 .

# Run using Docker
# Usage: just docker-run day-01 part1
docker-run day part: docker-build
    @echo "🐳 Running {{day}}-{{part}} inside Docker..."
    docker run --rm aoc-2025 {{day}}-{{part}}

cover day:
    @echo "☂️  Generating coverage for {{day}}..."
    cargo tarpaulin -p {{day}}
    @echo "✅ Report generated!"

cover-all:
    @echo "☂️  Generating FULL 2025 coverage report..."
    cargo tarpaulin --workspace --out Html --output-dir coverage/all
    @echo "✅ Unified Report: coverage/all/tarpaulin-report.html"
    xdg-open coverage/all/tarpaulin-report.html || open coverage/all/tarpaulin-report.html || true

# Usage: just bench day-08
bench day:
    @echo "🔥 Benchmarking {{day}}..."
    cargo bench -p {{day}}