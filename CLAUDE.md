# MySocial Rust SDK Development Guide

## Build Commands
- Build all crates: `cargo build`
- Build a specific crate: `cargo build -p mysocial-sdk-types`
- Run tests: `cargo test`
- Run a single test: `cargo test -p mysocial-sdk-types test_name`
- Check compilation: `cargo check`
- Generate docs: `cargo doc --open`

## Code Style Guidelines
- **Formatting**: Use `rustfmt` for consistent code formatting
- **Imports**: Group imports by std, external crates, then internal modules
- **Naming**: Use snake_case for variables/functions, CamelCase for types/traits
- **Error Handling**: Prefer `Result<T, E>` with custom error types and error propagation
- **Documentation**: All public APIs must have doc comments with examples
- **Types**: Use strong typing, avoid `unwrap()` in production code, prefer `?` operator
- **Testing**: Unit tests in the same file, integration tests in `tests/` directory
- **Feature Flags**: Use feature flags for optional functionality

## Project Structure
- Core types in `mysocial-sdk-types`
- Cryptographic functions in `mysocial-crypto`
- GraphQL client in `mysocial-graphql-client`
- Transaction building in `mysocial-transaction-builder`