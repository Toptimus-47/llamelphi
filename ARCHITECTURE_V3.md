# ELITE System Architecture V3 (Rust BL & Dart UI)

## 1. Vision
Achieve maximum efficiency and type safety by consolidating the **Business Logic (BL) into Rust** and the **User Experience (UX) into Dart/Flutter**. This 2-layer model eliminates the Python orchestration overhead and provides a high-performance, native-speed agent.

## 2. Layered Structure

### Layer 1: Presentation (Dart/Flutter)
- **Role**: High-fidelity UI, Async State Management, Stream Processing.
- **Key Tech**: Riverpod, fl_chart, flutter_markdown.
- **Protocol**: HTTP/1.1 (axum/tokio) with NDJSON (Newline Delimited JSON) Streaming.

### Layer 2: Business Logic & Core (Rust)
- **Role**: API Server, Agent State Machine, RAG, Inference (In-process).
- **Key Tech**: axum (Web Server), tokio (Async Runtime), llama-cpp-rs (Inference), faiss-rs (Vector Search).
- **Direct Control**: The Rust core directly handles LLM orchestration and data retrieval without intermediate scripts.

## 3. Implementation Plan (The "Pure Rust" Pivot)
1. **Core Overhaul**: Transform `elite_core` from a library into a binary API server.
2. **Logic Migration**: Port the Agent state machine and RAG logic from Python legacy to Rust.
3. **Inference Unification**: Use Rust-native LLM bindings (llama-cpp-rs/Candle).
4. **Dart Adaptation**: Point `EliteApiService` in Flutter to the Rust server.

## 4. Completed Milestones
- [x] Legacy Python Backup (`backups/legacy_python/`).
- [x] Flutter Presentation Framework (Refined for session history).
- [ ] Rust Core Server Implementation (In Progress).
