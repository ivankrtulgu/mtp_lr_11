# Stage 1: Builder
FROM golang:1.22-alpine AS builder

WORKDIR /build

# Install ca-certificates and create non-root user
RUN apk add --no-cache ca-certificates && \
    adduser -D -u 1001 -g 1001 appuser

# Copy go mod files first for better caching
COPY go.mod ./
RUN go mod download

# Copy source code
COPY . .

# Run tests - build will fail if tests don't pass
RUN go test -v ./...

# Build static binary with stripped debug info
RUN CGO_ENABLED=0 GOOS=linux go build -ldflags="-s -w" -o /build/app ./main.go

# Stage 2: Final
FROM scratch

# Copy CA certificates from builder
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy user/group files for non-root execution
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

# Copy the binary
COPY --from=builder /build/app /app

# Set user to non-root
USER appuser

# Expose port (documentational)
EXPOSE 8080

# Set entrypoint
ENTRYPOINT ["/app"]
