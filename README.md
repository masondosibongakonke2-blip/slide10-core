# Slide 10 Core

Offline-first edge orchestration engine for partitioned and constrained networks.

Slide 10 keeps coordination continuity alive when central links degrade or fail. It adapts local operating thresholds, filters low-value edge traffic, protects offline artifacts with hash and signature checks, persists local state, and reconciles verified records when connectivity returns.

## What is implemented

- Adaptive modes: `3f+1` → `2f+1` → `1f+1`
- Telemetry-driven mode suggestion
- Edge admission filter
- SHA-256 content-addressed integrity
- Ed25519 signatures
- SQLite persistence
- In-memory mesh transport (test)
- Deterministic reconciliation with reject path
- Full runnable demo

## What is not implemented yet

- Real mesh networking (Bluetooth / Wi-Fi Direct / LoRa)
- Hardware radio integrations
- Formal production security audit

## Quick start

```bash
cargo run
