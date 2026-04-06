# Rust Axum Web Service with Docker

A production-ready Rust web service using `axum` framework with Docker containerization.

## Project Structure

```
Task3/
├── Cargo.toml          # Dependencies and project metadata
├── Cargo.lock          # Locked dependency versions for reproducible builds
├── src/
│   ├── lib.rs          # Library: handlers, router, config, tests
│   └── main.rs         # Binary entrypoint: server setup and graceful shutdown
├── tests/
│   └── integration.rs  # Integration tests (no network required)
├── Dockerfile          # Multi-stage production Docker build
├── .dockerignore       # Excluded files from Docker context
└── README.md           # This file
```

## Features

- **Axum 0.7** web framework with Tokio async runtime
- **JSON logging** via `tracing` + `tracing-subscriber`
- **Request logging middleware** with method, URI, status code, and latency
- **Graceful shutdown** on SIGINT/SIGTERM (10s drain timeout)
- **Environment-based configuration** with validation (`APP_HOST`, `APP_PORT`, `RUST_LOG`)
- **Non-root container** execution (UID 10001)
- **Distroless runtime** image for minimal attack surface
- **12 tests**: 9 unit tests + 3 integration tests

## API Endpoints

| Method | Path      | Status | Response                              |
|--------|-----------|--------|---------------------------------------|
| GET    | `/`       | 200    | `{"message":"ok","version":"0.1.0"}`  |
| GET    | `/health` | 200    | `{"status":"healthy"}`                |

## Local Development

### Run Tests

```bash
cargo test
```

All tests are asynchronous, require no network or external dependencies, and run via `tower::ServiceExt::call`.

### Run Locally

```bash
# Default: 0.0.0.0:3000, RUST_LOG=info
cargo run

# Custom configuration
APP_HOST=127.0.0.1 APP_PORT=8080 RUST_LOG=debug cargo run
```

### Check Endpoints

```bash
# Root endpoint
curl http://localhost:3000/

# Health endpoint
curl http://localhost:3000/health
```

### Graceful Shutdown

Send SIGINT (Ctrl+C) or SIGTERM to trigger graceful shutdown:

```bash
# In another terminal:
kill -SIGTERM <pid>
# or
curl -X POST http://localhost:3000/shutdown  # (if implemented)
```

### View Logs

Logs are JSON-formatted. Example output:

```json
{"timestamp":"2026-04-06T10:00:00.000000Z","level":"INFO","message":"Starting server on 0.0.0.0:3000"}
{"timestamp":"2026-04-06T10:00:05.000000Z","level":"INFO","message":"response completed","status":200,"latency_ms":1}
```

## Docker

### Build Image

```bash
docker build -t rust-axum-app:latest .
```

### Run Container

```bash
docker run -d \
  --name axum-service \
  -p 3000:3000 \
  -e APP_HOST=0.0.0.0 \
  -e APP_PORT=3000 \
  -e RUST_LOG=info \
  rust-axum-app:latest
```

### Verify Container

```bash
# Check endpoints
curl http://localhost:3000/
curl http://localhost:3000/health

# Check health status
docker inspect --format='{{.State.Health.Status}}' axum-service

# View logs
docker logs axum-service
docker logs -f axum-service  # follow mode
```

### Graceful Shutdown (Container)

```bash
docker stop --time=10 axum-service
```

This sends SIGTERM and waits up to 10 seconds for graceful shutdown.

### Remove Container

```bash
docker rm -f axum-service
docker rmi rust-axum-app:latest
```

## Technical Details

### 1. Docker Layer Caching

The Dockerfile uses multi-stage builds with strategic layer ordering:
- **Stage 1 (builder)**: Copies `Cargo.toml` and `Cargo.lock` first, then runs a dummy build to cache dependencies. The actual source is copied afterward, so dependency resolution and compilation of unchanged crates is cached across builds.
- **Stage 2 (runtime)**: Copies only the compiled binary from the builder stage, resulting in a minimal final image (~20MB vs ~2GB for a full Rust toolchain image).

### 2. Distroless Image Choice

`gcr.io/distroless/cc-debian12:nonroot` is chosen because:
- **Minimal attack surface**: Contains only the application and its runtime dependencies (no shell, package manager, or debug tools)
- **Smaller image size**: ~20MB vs Alpine's ~50MB+ with dependencies
- **Non-root by default**: The `:nonroot` tag runs as UID 10001, following CIS Docker Benchmark recommendations
- **C/C++ runtime support**: Includes `libc` and `libgcc` needed for dynamically linked Rust binaries

### 3. Graceful Shutdown Implementation

The server handles shutdown via:
- **Signal handlers**: Listens for `SIGINT` (Ctrl+C) and `SIGTERM` (Docker stop) using `tokio::signal`
- **Cross-platform**: On Unix, handles both SIGINT and SIGTERM; on Windows, only SIGINT is supported
- **tokio::select!**: Races both signal futures; whichever completes first triggers shutdown
- **Connection draining**: `axum::serve` with `with_graceful_shutdown()` stops accepting new connections and waits for active requests to complete (up to 10 seconds)

### 4. Test Isolation from Network

Integration tests use `tower::ServiceExt::call` instead of real HTTP requests:
- **In-memory testing**: Requests are constructed as `axum::http::Request<Body>` and passed directly to the router
- **No network stack**: Bypasses TCP/IP, eliminating flakiness from port conflicts, firewalls, or network latency
- **Deterministic**: Tests run synchronously with predictable timing
- **No mocks needed**: The actual axum handlers are invoked, testing real routing, serialization, and middleware behavior

## Configuration

| Variable   | Default    | Description                        |
|------------|------------|------------------------------------|
| `APP_HOST` | `0.0.0.0`  | Host address to bind               |
| `APP_PORT` | `3000`     | Port number (must be 1-65535)      |
| `RUST_LOG` | `info`     | Logging level (trace/debug/info/warn/error) |

## Dependencies

| Crate              | Version | Purpose                          |
|--------------------|---------|----------------------------------|
| `axum`             | 0.7     | Web framework                    |
| `tokio`            | 1       | Async runtime                    |
| `serde`            | 1       | Serialization/deserialization    |
| `serde_json`       | 1       | JSON support                     |
| `tracing`          | 0.1     | Structured logging               |
| `tracing-subscriber`| 0.3    | Log formatting (JSON output)     |
| `tower-http`       | 0.6     | Request logging middleware       |
| `http-body-util`   | 0.1     | Body utilities for tests         |
| `tower`            | 0.5     | Service testing (dev dependency) |

## License

MIT
