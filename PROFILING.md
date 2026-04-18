# Profiling with Callgrind

Guide for analyzing hot paths in this project when `perf`-based tools
(samply, cargo-flamegraph, perf) aren't available — e.g., inside
GitHub Codespaces / devcontainers where `/proc/sys/kernel/perf_event_paranoid`
is locked at 2 and `/proc` is mounted read-only.

## Install

Debian / Ubuntu:

```bash
sudo apt-get update && sudo apt-get install -y valgrind
```

Verify:

```bash
valgrind --version
which callgrind_annotate
```

## Prerequisites

1. The repo already defines a dedicated `profiling` cargo profile in
   `Cargo.toml` that inherits from `release` and adds debug symbols —
   needed so callgrind can map instructions back to `file.rs:line`.
   Shipping `release` builds stay stripped and small.

2. Build the profiling binary:

   ```bash
   cargo build --profile profiling
   ```

   The binary lands at `target/profiling/rng_f`.

3. **Use a reduced input size.** Callgrind emulates every instruction and
   runs roughly 20–40× slower than native. For this project, `1000 1000`
   finishes in ~30s under callgrind; full `10000 10000` would take
   15–20 minutes.

## Basic run

```bash
valgrind --tool=callgrind --callgrind-out-file=callgrind.out \
  ./target/profiling/rng_f 1000 1000 number
```

Produces `callgrind.out` with per-function instruction counts.

## With cache simulation

Adds another ~2× slowdown; use only when you need to know whether cache
misses, not just instruction count, are driving runtime.

```bash
valgrind --tool=callgrind --cache-sim=yes --callgrind-out-file=callgrind.out \
  ./target/profiling/rng_f 1000 1000 number
```

## Reading the output

```bash
callgrind_annotate callgrind.out --threshold=95 | head -50
```

- `--threshold=95` shows enough functions to cover 95% of instructions.
- With `--cache-sim=yes`, add `--show=Ir,D1mr,D1mw,DLmr,DLmw` to include
  cache-miss columns.
- Each row: `Ir (% of total) file:function` — highest `Ir` is the hottest
  function.

## Caveats

- **Don't trust callgrind's wall time.** The "Generation took N ms"
  your program prints runs under valgrind's emulator, so it's much
  slower than native and not comparable between runs with different
  valgrind options. Use instruction count (`Ir`) for ranking
  hot functions and a separate native `cargo run --release` for
  real wall-time measurements.
- **Cache sim results are simulated**, based on generic cache
  parameters, not your actual CPU's cache. Useful for ranking, not
  absolute miss rates.

## Measuring native wall time

Independent of callgrind — the program prints its own generation time:

```bash
for i in 1 2 3; do
  cargo run --release --quiet -- 10000 10000 number 2>&1 | grep "took"
done
```

Run 3× to smooth out noise.
