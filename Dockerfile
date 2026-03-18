ARG IMAGE=rust:1.94.0-slim-bookworm

# Build
FROM ${IMAGE} AS planner
RUN cargo install cargo-chef

# Set work directory
WORKDIR /app
COPY . .

# Prepare a build plan ("recipe")
RUN cargo chef prepare --recipe-path recipe.json

FROM ${IMAGE} AS build
RUN cargo install cargo-chef

# Install postgres library
RUN apt-get update && apt-get install libssl-dev pkg-config libpq-dev clang curl -y

# Set consistent working directory
WORKDIR /app

# Copy the build plan from the previous Docker stage
COPY --from=planner /app/recipe.json recipe.json

# Build dependencies - this layer is cached as long as `recipe.json`
# doesn't change.
RUN cargo chef cook --release --recipe-path recipe.json

# Build the whole project
COPY . .

# Build application
RUN cargo build --release

# RUNTIME
FROM ${IMAGE} AS runtime

# Install dependency (Required by diesel)
RUN apt-get update && apt-get install curl libpq-dev -y

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Setup working directory
WORKDIR /ahmard/apps/refinery-procurement

# Create uploads directory
RUN mkdir -p resources/static/uploads

# Copy configuration files
COPY diesel.toml diesel.toml

# Copy resources folder
COPY resources resources

# Copy our built binaries
COPY --from=build /app/target/release/auth-service /usr/local/bin/auth-service
COPY --from=build /app/target/release/admin-service /usr/local/bin/admin-service
COPY --from=build /app/target/release/catalog-service /usr/local/bin/catalog-service
COPY --from=build /app/target/release/procurement-service /usr/local/bin/procurement-service
COPY --from=build /app/target/release/worker-service /usr/local/bin/worker-service

# Default command (will be overridden by docker-compose)
CMD ["auth-service"]