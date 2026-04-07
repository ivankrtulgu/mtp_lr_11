# CI/CD Pipeline — Multilingual Monorepo (Python, Go, Rust)

## 📋 Overview

This GitHub Actions workflow automatically **lints**, **tests**, builds, and pushes Docker images for three services:

| Service | Path | Language | Framework |
|---------|------|----------|-----------|
| Python  | `Task1/` | Python 3.12 | FastAPI + SQLAlchemy |
| Go      | `Task_hard1/` | Go 1.22 | net/http (static binary → scratch) |
| Rust    | `Task3/` | Rust 1.83 | Axum + Tokio |

---

## 🔐 Required GitHub Secrets

Navigate to **Settings → Secrets and variables → Actions** and add:

| Secret Name | Description | How to Obtain |
|-------------|-------------|---------------|
| `DOCKERHUB_USERNAME` | Your Docker Hub username | Found on your [Docker Hub profile](https://hub.docker.com/) |
| `DOCKERHUB_TOKEN` | Docker Hub Personal Access Token | Docker Hub → Account Settings → Security → New Access Token |

> ⚠️ **Never** hardcode credentials. The workflow reads them exclusively from `${{ secrets.* }}`.

---

## 🚀 How to Verify the Pipeline

### 1. Check Build Status
- Go to your repository → **Actions** tab.
- Click on any workflow run named `CI/CD Pipeline`.
- Each matrix job (`python`, `go`, `rust`) runs **in parallel**.
- Green ✓ = success; Red ✗ = failure (pipeline stops on first test failure).

### 2. Find Pushed Images in Docker Hub
After a successful **push to `main`**:
```bash
# Pull the latest images
docker pull <DOCKERHUB_USERNAME>/fastapi-app:latest
docker pull <DOCKERHUB_USERNAME>/go-app:latest
docker pull <DOCKERHUB_USERNAME>/rust-app:latest

# Or pull by specific commit SHA
docker pull <DOCKERHUB_USERNAME>/fastapi-app:abc1234
```

Verify on Docker Hub web: **Repositories** → look for `fastapi-app`, `go-app`, `rust-app`.

---

## 🛡️ How This Pipeline Ensures Code Quality

| Stage | What It Does | Why It Matters |
|-------|--------------|----------------|
| **Lint** | `flake8` (Python), `go vet` (Go), `cargo clippy` (Rust) | Catches syntax errors, unused imports, anti-patterns **before** build |
| **Test** | `pytest` / `go test -race` / `cargo test` | **Fails fast** — if any test fails, the Docker image is **never built or pushed** |
| **Docker Build** | Multi-stage builds with `.dockerignore` | Only production artifacts enter the image; no `venv`, no `target/`, no source leaks |
| **Tagging** | `latest` + `<short-sha>` | Every image is **traceable** to an exact commit — rollback is `docker pull <image>:<sha>` |
| **Cache** | GitHub Actions Cache (Rust deps) + GHA cache (Docker layers) | Speeds up builds from 20 min → ~2 min for unchanged dependencies |

### Guarantees
1. ✅ Only **tested** code produces Docker images.
2. ✅ Only images from **`push`** events are pushed (PRs only build & test).
3. ✅ Every image is **immutable** and auditable via SHA tag.
4. ✅ Parallel matrix strategy means **faster feedback** — no sequential waiting.

---

## 📁 Project Structure

```
Task_hard3/
└── .github/
    └── workflows/
        └── main.yml          ← CI/CD pipeline (this file)

Task1/                        ← Python/FastAPI service
Task_hard1/                   ← Go service (multi-stage → scratch)
Task3/                        ← Rust/Axum service (cached deps)
```

---

## ⚙️ Trigger Conditions

| Event | Branch | Action |
|-------|--------|--------|
| `push` | `main` | Lint → Test → Build → **Push** to Docker Hub |
| `pull_request` | `main` | Lint → Test → Build (no push) |

---

## 🧪 Local Testing (Before Push)

```bash
# Python
cd Task1 && pip install -r requirements.txt && pytest test_main.py -v

# Go
cd Task_hard1 && go test -v ./...

# Rust
cd Task3 && cargo test --verbose
```

---

## 📊 Matrix Strategy Breakdown

```yaml
strategy:
  matrix:
    service: [python, go, rust]
  fail-fast: true
```

- **3 parallel jobs** spawn simultaneously.
- `fail-fast: true` → if one service fails tests, others are cancelled immediately (saves CI minutes).
- Each job gets its own language runtime, linter, and Docker build context.
