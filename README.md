# Fraud Engine Zero

Fast, deterministic transaction fraud detection engine built in Rust. It uses the **Z3 SMT solver** for formal rule verification, **gRPC** for low-latency ingestion, and **WebSockets** for real-time dashboard streaming.

---

## Demo

Watch the operational demo on Streamable:
👉 **[Watch Video Demo](https://streamable.com/4pzz...)**

*Highlights:* Receiving transactions over gRPC, running risk validation against Rust/Z3 rules, and streaming live metrics to the dashboard via WebSockets with sub-millisecond overhead.

---

## Why this exists

Imperative rule engines often get messy as rule sets grow, leading to rule conflicts and silent edge-case bugs. 

**Fraud Engine Zero** handles fraud detection by treating rule evaluation as a formal logic problem using an SMT Solver (Z3). Combined with Rust's memory safety and async execution model, it yields deterministic decisions at microsecond scale ($\mu s$) without trade-offs in correctness.

---

## Tech Specs

* **Sub-millisecond decisions:** Built on `tokio` for non-blocking concurrent throughput.
* **Formal logic constraints:** Powered by `z3` (SMT solver) to prevent false positives and conflicting rule logic.
* **gRPC Ingestion:** Low-overhead binary transport via `tonic` / Protocol Buffers.
* **Live Streaming:** Real-time state push to the dashboard using WebSockets (`axum` + `tokio-tungstenite`).
* **Telemetry:** Native Prometheus exporter (`/metrics`).

---

## System Overview

```text
[ Bank / Payment Service ]
          │ (gRPC)
          ▼
┌─────────────────────────────────────────┐
│        Fraud Engine Zero (Rust)         │
│                                         │
│   Tokio Runtime ───► Z3 Logic Solver    │
│         │                  │            │
│         ▼                  ▼            │
│   Audit Logs        Prometheus Metrics  │
└─────────────────────────────────────────┘
          │
          │ (WebSockets)
          ▼
[ Real-Time Frontend ]

Stack
Language: Rust (2021 edition)
Async Core: Tokio
gRPC: tonic / prost
SMT Engine: z3
Web Server: axum
Metrics: Prometheus
Frontend: React + Tailwind CSS
Quickstart
Prerequisites
Rust toolchain (stable)
C++ compiler (clang/gcc), protobuf-compiler, and z3 development headers installed on your system.
Build and Run

1 Clone repo:

git clone [https://github.com/eduardoolivera142345-byte/fraud-engine-zero](https://github.com/eduardoolivera142345-byte/fraud-engine-zero)
cd fraud-engine-zero

2 Build optimized release binary:

cargo build --release

3 Run:

cargo run --release

Default services ports:
gRPC Server: 0.0.0.0:50051
WebSocket Server: 0.0.0.0:8080
Prometheus Metrics: 0.0.0.0:9090/metrics
License
MIT License. Created by Eduardo Oliveira.