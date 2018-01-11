RayTracer
===

A ray tracer in rust implementing gravitational lensing. Based on a Ray Tracer implementation by @ranveeraggarwal

## Running

```bash
cargo run --release --example gravity
```

For faster performance try turning down the image size or samples. You can configure this in `examples/gravity.rs` by
modifying the following constants:

```rust
let nx: u64 = 1200;
let ny: u64 = 800;
let ns: u64 = 16;
```

In this code, `nx` is the number of horizontal pixels, `ny` is the number of vertical pixels, and `ns` is the number of
samples per pixel.
