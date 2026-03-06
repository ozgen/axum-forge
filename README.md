# axum-forge

`axum-forge` is an opinionated project for building a production-ready Axum + SQLx REST API starter.

The goal is to provide a clean, explicit, and maintainable starting point for Rust backend services using boring, proven defaults.

## Current Status

This repository currently contains the first manually built template application.

The template lives under:

```text
template/axum-forge-template
```

This template will become the reference output for the future `cargo axum-forge` CLI.

## Goals

- Axum-based REST API starter
- SQLx with PostgreSQL
- clear project structure
- explicit configuration
- consistent JSON error responses
- health and readiness endpoints
- migrations
- integration testing patterns
- Makefile for common development tasks

## Non-Goals

- not a web framework
- not a replacement for Axum or SQLx
- not a generic abstraction layer
- not a full authentication server
- not a “support everything” generator

## Development Approach

This project follows a template-first approach:

1. Build the golden output service manually
2. Stabilize the structure and defaults
3. Extract reusable patterns only when proven
4. Build the CLI last

## Repository Structure

```text
axum-forge/
├── template/
│   └── axum-forge-template/
└── README.md
```

## Template Structure

```text
src/
  main.rs
  app.rs
  error.rs
  state.rs
  telemetry.rs

  config/
    mod.rs
    app.rs
    db.rs

  modules/
    mod.rs
    error.rs

    health/
      mod.rs
      handlers.rs

    items/
      mod.rs
      dto.rs
      handlers.rs
      model.rs
      repo.rs
```

## Run the Template

Start PostgreSQL with Docker Compose from the template directory:

```bash
cd template/axum-forge-template
docker compose up -d
```

Run the application:

```bash
cargo run
```

Test endpoints:

```bash
curl http://localhost:3000/healthz
curl http://localhost:3000/readyz
curl http://localhost:3000/api/v1/items
```

## Vision

The long-term goal is a Cargo subcommand like:

```bash
cargo axum-forge new my-service
```

which scaffolds a project based on the manually refined template.

## License

MIT
