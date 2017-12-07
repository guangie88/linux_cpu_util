# `linux_cpu_util`

To calculate CPU utilization for Linux on `Rust`. Requires nightly compiler due
to usage of `#![feature(slice_patterns)]`.

To solve https://rosettacode.org/wiki/Linux_CPU_utilization without using
`.unwrap()`.

## How to Build

```bash
cargo build --release
```

## How to Run

CPU usage refreshes on every second interval.

```bash
./target/release/linux_cpu_util 1
```
