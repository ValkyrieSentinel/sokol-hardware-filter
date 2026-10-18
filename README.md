# Sokol Hardware Filter

Zero-allocation, lock-free, biomimetic hardware filtering core in `#![no_std]` Rust.

This module acts as the "Hardware Retina" for the Sokol ecosystem. It performs O(1) packet state resolution and foveated anomaly detection at the edge, without relying on an OS allocator.

## Architecture

Inspired by biological vision, the filter operates in two phases:
1. Fast Temporal Masking (Peripheral Vision): Uses bitwise operations to instantly drop noise in O(1) time.
2. Deep Scan (Foveal Focus): Triggered only for complex anomalies.

## Core Components

- **QuantumStateVector**: Resolves multi-hypothesis states into a single decision byte in O(1) using `trailing_zeros()`. No branching.
- **FastCffBuffer**: Strictly lock-free, single-producer ring buffer. Zero mutexes, zero allocations.
- **DualFoveaPipeline**: Two-layer event processing (fast bitwise masking vs deep scan).
- **PectenTelemetry**: Lock-free atomic counters for pass/drop metrics.

## Build

For Bare-Metal / Embedded:
cargo build --release --target thumbv7em-none-eabihf

For eBPF (Kernel Space):
cargo build --release --target bpfel-unknown-none -Z build-std=core

For Host Testing:
cargo test --release

## License

MIT. Part of the ValkyrieSentinel Sokol Ecosystem.