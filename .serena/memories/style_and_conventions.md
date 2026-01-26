# Style and Conventions

## Language and Tooling
- **Rust**: Use the latest stable Rust (2024 edition).
- **Cargo**: Standard build and dependency management.

## Code Style
- **Formatting**: Adhere to `rustfmt` defaults.
- **Linting**: Use `clippy` and address all warnings. Use `cargo clippy -- -D warnings`.
- **Naming**: Follow standard Rust naming conventions (snake_case for functions/variables, PascalCase for types).
- **Documentation**: All public symbols (structs, enums, functions, modules) must have comprehensive rustdoc comments. Include examples where appropriate.

## Patterns and Error Handling
- **Idiomatic Rust**: Prefer standard library solutions and established patterns.
- **Error Handling**: Use `Result<T, E>` for fallible operations. Avoid `unwrap()` and `panic!` in library code. Use `anyhow` or `thiserror` for better error management.
- **Performance**: Focus on zero-cost abstractions and efficient data structures. Avoid unnecessary allocations.

## Clustering Logic
- Time-based clustering uses circular K-means to handle wrap-around at midnight.
- Distance-based clustering uses linear K-means.
- The "Elbow Method" is used to determine the optimal number of clusters (k).
