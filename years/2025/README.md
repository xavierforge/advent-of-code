# Advent of Code 2025: Rust DevOps Edition 🦀

[![AoC 2025 CI](https://github.com/xavierforge/advent-of-code/actions/workflows/ci.yml/badge.svg)](https://github.com/xavierforge/advent-of-code/actions/workflows/ci.yml)
![Rust Version](https://img.shields.io/badge/rust-1.89%2B-orange)
![Edition](https://img.shields.io/badge/edition-2024-red)
![Docker](https://img.shields.io/badge/container-ready-blue)

> **Not just solving puzzles; building a production-grade engineering pipeline.**

## 📖 About The Project

This repository contains my solutions for [Advent of Code 2025](https://adventofcode.com/2025). 

However, the primary goal of this project is not merely algorithmic problem solving. I am using this challenge as a **sandbox to demonstrate modern DevOps and MLOps practices** using the Rust ecosystem. 

Every solution is treated as a microservice: containerized, tested, and deployed via a strict CI/CD pipeline.

## 🏗️ Architecture & Tech Stack

This project is structured as a **Rust Monorepo (Workspace)** to simulate a real-world engineering environment.

- **Language:** Rust (v1.89, **2024 Edition**)
- **Build System:** Cargo Workspace (Monorepo)
- **Containerization:** Docker (Multi-stage builds, Distroless images)
- **CI/CD:** GitHub Actions (Automated Testing, Linting, Caching)
- **Task Runner:** `just` (Command automation)
- **Testing:** `cargo-nextest` (Fast, parallel test execution)
- **Dependency Management:** `cargo-chef` (Docker layer caching optimization)

### Directory Structure

```text
.
├── .github/workflows  # CI/CD Pipeline definitions
├── crates/
│   └── common         # Shared logic & utilities (DRY principle)
├── years/
│   └── 2025/          # Daily solutions (Member crates)
│       ├── day-01/
│       ├── day-02/
│       └── ...
├── Dockerfile         # Production-ready image definition
├── Justfile           # Automation scripts
└── Cargo.toml         # Workspace configuration
````

## 🚀 DevOps Highlights

### 1\. Reproducibility via Docker

"It works on my machine" is not acceptable. This project includes a production-grade `Dockerfile` that uses **multi-stage builds** and **`cargo-chef`** to create minimal, highly optimized images.

To run any solution in a containerized environment:

```bash
just docker-run day-01 1
```

### 2\. Automated Quality Assurance (CI)

A strict GitHub Actions pipeline runs on every push:

  - **Linting:** Enforced via `clippy` (deny warnings) and `rustfmt`.
  - **Testing:** Unit tests run via `cargo-nextest`.
  - **Caching:** Aggressive usage of `Swatinem/rust-cache` to speed up CI times.

### 3\. Developer Experience (DX)

I use `just` to abstract complex cargo commands, ensuring a smooth workflow for any contributor (or reviewer).

## 🛠️ Getting Started

### Prerequisites

  - Rust (v1.89+)
  - Docker
  - Just (`cargo install just`)

### Setup

Initialize the environment and install necessary tools:

```bash
just setup
```

### Daily Workflow

The entire lifecycle is automated:

1.  **Create a new day** (Generates boilerplate from template):
    ```bash
    just create day-01
    ```
2.  **Develop with live feedback** (Watch mode for TDD):
    ```bash
    just work day-01 part1
    ```
3.  **Run the solution (Local):**
    ```bash
    # Usage: just run <day> <part>
    just run day-01 part1
    ```
4.  **Run inside Docker (Production simulation):**
    ```bash
    just docker-run day-01 part1
    ```
5.  **Pre-push Check (Local CI):**
    ```bash
    just ci
    ```

## 📊 Progress (2025)

| Day | Part 1 | Part 2 | DevOps Focus |
| :---: | :---: | :---: | :--- |
| 01 | ⭐ | ⭐ | Infrastructure Setup, Dockerfile, CI Pipeline |
| 02 | ⭐ | ⭐ | Observability & Structured Logging (`tracing`, `test-log`) |
| 03 |   |   | *Pending* |
| ... | ... | ... | ... |
