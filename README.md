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

- OS process
  - out of scope of actix
- [`System`](https://docs.rs/actix/latest/actix/struct.System.html)
  - Usually, only one in process
  - Only for special reason, like test, there are multiple `System`s.
  - Between different `System`s, `Actor`s cannot send messages.
- [`Arbiter`](https://docs.rs/actix/latest/actix/struct.Arbiter.html)
  - Represents a OS thread
  - Each `Arbiter` is bound to one and only one `System`
  - `System` can hold multiple `Arbiter`s. But in this benchmark, we only use one `Arbiter`.
- [`Actor`](https://docs.rs/actix/latest/actix/trait.Actor.html)
  - Encapsulates state and behavior
  - Communicates with other `Actor`s by sending and receiving messages
  - Each `Actor` is bound to one and only one `Arbiter`
