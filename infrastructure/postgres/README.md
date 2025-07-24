# Infrastructure: PostgreSQL Module

This module provides the concrete implementation for data persistence using PostgreSQL in a Rust DDD architecture. It acts as the data source for repository traits defined in the domain layer, following the Dependency Inversion principle.

It is responsible for:
- Establishing and managing a connection to PostgreSQL
- Mapping between domain models and database rows
- Executing SQL queries to persist and retrieve domain data
- Implementing the repository traits defined in the domain layer

## Role in Architecture

This module belongs to the Infrastructure Layer of the DDD architecture. It fulfills the contracts defined in the Domain Layer by providing real implementations using SQL queries via the `sqlx` crate.

It depends on:
- `domain::repositories::*` — for repository trait definitions
- `domain::models::*` — for core business entities

It does **not** contain any domain logic. It is purely focused on data access and persistence.

## Data Flow

1. Application or domain services call repository traits like `CarRepository`.
2. This module provides the actual `PostgresCarRepository` implementation.
3. SQLx is used to execute raw SQL queries on the Postgres database.
4. Query results are mapped back into domain entities.
5. The domain layer remains unaware of the underlying storage technology.


## Integration

- The infrastructure layer exposes a constructor (e.g., `PostgresInfrastructure::new(pool)`) to create all repository instances.
- These implementations are passed into the application layer as boxed trait objects (`Box<dyn CarRepository>`).
- Dependency injection is performed manually or using DI crates like `shaku`.

## Testing

- Unit tests use `#[cfg(test)]` modules within each repository.
- Integration tests may use a Dockerized Postgres container to run end-to-end tests.
- `sqlx::query!` and `sqlx::query_as!` macros validate SQL queries at compile time.
- Optionally, use SQLx offline mode for CI/CD environments.

## Design Notes

- Domain models are used directly — database structs or DTOs are avoided.
- All queries use raw SQL with `sqlx` to ensure clarity and performance.
- No domain logic is allowed in this module.
- Errors are converted into a custom `DbError` enum.
- Indexes and query plans are optimized for critical paths.
- `mappers/` folder isolates conversion logic between SQL rows and domain entities.

## Future Plans

- Add tracing and metrics for all DB operations
- Introduce caching layer using Redis for read-heavy queries
- Adopt `sqlx-migrate` or `refinery` for migrations
- Explore CQRS/event sourcing patterns to separate read/write concerns
- Optimize batch operations and pagination

## Notes for AI Agents

- This module implements repository traits. Do **not** define or invent new traits here.
- Only SQL, connection handling, and model mapping belong here.
- Always use `domain::{entity}::*` as return values from repository functions.
- Follow DDD rules: Infrastructure depends on Domain. Never the opposite.
- Do not place domain logic or business validation in this module.
- Use `sqlx::query_as!` macros with typed domain entities when writing SQL.

