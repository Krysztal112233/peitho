set dotenv-load := true

# Generate SeaORM entities into peitho-database models module.
gen-models:
    @if [ -z "${DATABASE_URL:-}" ]; then echo "DATABASE_URL is required"; exit 1; fi
    @mkdir -p crates/peitho-database/src/models
    sea-orm-cli generate entity -u "$DATABASE_URL" -o crates/peitho-database/src/models --with-serde both
