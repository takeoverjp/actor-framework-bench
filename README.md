# actor-framework-bench
Benchmark Rust actor frameworks with same application

## Target Frameworks

### [Actix](https://actix.rs/docs/actix/getting-started)

You can run the Actix example with:

```
cargo run --bin actix
```

### Concepts

actix has 3 types for controlling concurrency

- process
  - out of scope of actix
- `System`
  - Usually, only one in process
  - Only for special reason, like test, there are multiple `System`s.
- `Arbiter`
  - 
- `Actor`
