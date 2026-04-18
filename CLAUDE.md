# CLAUDE.md

## Profiling

For hot-path analysis, see [PROFILING.md](PROFILING.md). It documents the
callgrind workflow used in this repo — `perf`-based profilers (samply,
cargo-flamegraph, perf) are typically blocked in devcontainer
environments because `/proc/sys/kernel/perf_event_paranoid` is locked
and `/proc` is read-only.
