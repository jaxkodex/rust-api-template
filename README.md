# Rust API template

A minimal Rust backend using Axum, Tokio and PostgreSQL. It includes environment-based configuration, SQLx migrations, liveness and readiness checks, optional browser CORS, request tracing, graceful shutdown, Docker, and GitHub Actions.

## Requirements

- Stable Rust with `cargo`, `rustfmt` and `clippy`.
- PostgreSQL. The integration tests use SQLx's per-test databases, so the database user also needs the `CREATEDB` privilege.

## Run locally

For example, start a local PostgreSQL server with Docker:

```sh
docker run -d --name template-pg -e POSTGRES_PASSWORD=postgres -e POSTGRES_DB=template_dev -p 5432:5432 postgres:16
```

Set the environment variables in your shell and start the API:

```sh
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/template_dev
cargo run --locked
```

In PowerShell, use `$env:DATABASE_URL = 'postgres://postgres:postgres@localhost:5432/template_dev'` instead of `export`. The app reads the process environment; `.env.example` is a reference and is not loaded automatically. Migrations run at startup, and startup fails if the database is unavailable.

The server binds to `0.0.0.0:8080` by default:

- `GET /healthz` reports process liveness.
- `GET /readyz` runs a database query and returns 503 when PostgreSQL is unavailable.

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `DATABASE_URL` | Required | PostgreSQL connection URL. |
| `PORT` | `8080` | HTTP listen port. |
| `RUST_LOG` | `rust_api_template=info,tower_http=info` | Tracing filter. |
| `ALLOWED_ORIGINS` | Unset | Comma-separated HTTP(S) browser origins, e.g. `http://localhost:3000,https://example.com`. With no origins configured the API emits no CORS headers. |

Invalid ports or CORS origins fail startup. CORS is a browser policy, not authentication or authorization.

## Checks

With `DATABASE_URL` set to a PostgreSQL server whose user can create databases:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
```

The tests create temporary databases and apply the migrations. CI runs the same checks against PostgreSQL 16.

## Docker

```sh
docker build -t rust-api-template .
docker run --rm -p 8080:8080 --env-file .env rust-api-template
```

Create `.env` from `.env.example` first and set a database URL reachable **from inside the container**. `localhost` inside the container refers to the container itself, not the host. The `.env` file is excluded from Git and the build context.

## Starting a project from this template

1. Use GitHub's **Use this template** button, then rename the package in `Cargo.toml`, the binary and log target in `Dockerfile`, and the crate imports/log filter in `src/main.rs` and tests. Update the example environment file, dev container name and this README as needed.
2. Update `Cargo.lock` with `cargo generate-lockfile`, then add your routes, configuration and tests.
3. Add numbered `.sql` files under `migrations/`; SQLx embeds them at build time and applies them on startup. The initial migration is a harmless baseline.

This starter does not provide authentication, authorization, rate limiting, or a complete application data model; add the policies appropriate for your service.
