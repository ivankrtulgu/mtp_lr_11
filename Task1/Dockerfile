# Stage 1: Builder
FROM python:3.12-slim AS builder

WORKDIR /build

COPY requirements.txt .

RUN pip install --no-cache-dir --prefix=/install -r requirements.txt

# Stage 2: Runtime
FROM python:3.12-slim AS runtime

RUN groupadd -g 10001 appgroup && \
    useradd -u 10001 -g appgroup -s /bin/bash -m appuser

WORKDIR /app

COPY --from=builder /install /usr/local

COPY main.py .

RUN touch /app/sqlalchemy.db && \
    chown 10001:10001 /app/sqlalchemy.db && \
    chown -R 10001:10001 /app

USER 10001:10001

ENV APP_HOST=0.0.0.0
ENV APP_PORT=8000
ENV DATABASE_URL=sqlite:///./sqlalchemy.db

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')" || exit 1

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]
