# Dockerfile for rudof MCP server
# Optimized for Docker MCP Registry

FROM clux/muslrust:latest AS builder

WORKDIR /app

# Copy source code
COPY . .

# Build the rudof CLI binary (which includes the MCP server)
RUN cargo build --target x86_64-unknown-linux-musl --release -p rudof_cli

# Runtime image with Java and PlantUML
FROM eclipse-temurin:21-jre-alpine

# Install PlantUML
ARG PLANTUML_VERSION=1.2024.8
RUN mkdir -p /opt/plantuml && \
    wget -q -O /opt/plantuml/plantuml.jar \
    "https://github.com/plantuml/plantuml/releases/download/v${PLANTUML_VERSION}/plantuml-${PLANTUML_VERSION}.jar"

# Install graphviz for PlantUML diagrams
RUN apk add --no-cache graphviz fontconfig ttf-dejavu

# Set environment variables
ENV RUST_LOG="info,rudof_mcp=debug"
ENV PLANTUML="/opt/plantuml/plantuml.jar"

# Copy the binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rudof /usr/local/bin/rudof

# Switch to non-root user
USER 1000:1000

EXPOSE 8000

# MCP server entrypoint
ENTRYPOINT ["rudof"]
CMD ["mcp", "-t", "streamable-http", "-b", "0.0.0.0", "-p", "8000", "-r", "mcp","-n", "127.0.0.0/8", "-n", "::1/128", "-n", "172.16.0.0/12"]