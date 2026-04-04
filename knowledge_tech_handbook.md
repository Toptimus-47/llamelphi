# ELITE Strategic Technology Handbook (Modern Standards)

## 1. Flutter / Dart (Frontend Shell)
### Architectural Goal: Declarative, High-Performance UI.
- **Widget Patterns**:
  - Prefer **Composition over Inheritance**. Break UIs into small, single-responsibility widgets.
  - Use `const` constructors aggressively to optimize render trees.
  - Follow the **Constraints flow down, Sizes flow up** rule.
- **State Management (Riverpod)**:
  - Use `NotifierProvider` for logic and `FutureProvider`/`StreamProvider` for async data.
  - AI Refactoring Rule: Convert `StatelessWidget` to `ConsumerWidget` to access `WidgetRef`.
  - Use `ref.watch(provider)` for UI reactivity; `ref.read(provider)` for one-time actions (like button clicks).
- **Async & Streams**:
  - Use `async*` and `yield` for clean token streaming.
  - Always close `StreamController` in `dispose()` to prevent memory leaks.

## 2. Rust (Performance Core)
### Architectural Goal: Memory Safety and Zero-Cost Concurrency.
- **Async Patterns (Tokio)**:
  - Use `tokio::spawn` for non-blocking task execution.
  - Use `tokio::select!` to handle cancellation and timeouts efficiently.
  - Use `mpsc` (multi-producer, single-consumer) channels for inter-task communication.
- **ML Inference**:
  - **Candle**: High-level tensor manipulation, suitable for WASM and custom kernels. Manage `Device` (CPU/CUDA) explicitly.
  - **llama-cpp-rs**: Specialized for GGUF models. Use `mmap` for efficient large model loading.

## 3. FastAPI (The Intelligence Bridge)
### Architectural Goal: Low-Latency, Type-Safe API.
- **Streaming Responses**:
  - Use `StreamingResponse` with `async generator` functions for real-time LLM token delivery.
- **Best Practices**:
  - Use `Annotated[Type, Depends(func)]` for dependency injection.
  - Run CPU-bound tasks (Inference) in thread pools (via standard `def`) if they are not natively async.
  - Use `CORSMiddleware` to allow Flutter Desktop/Web clients to connect.

## 4. LangGraph (The Agent Brain)
### Architectural Goal: Stateful, Cyclic Workflows.
- **State Management**:
  - Define `TypedDict` or `Pydantic` as the source of truth for the Agent's state.
  - Use **Conditional Edges** for intelligent routing based on LLM output.
- **Advanced Patterns**:
  - **Reducers**: Control how node outputs merge into state (e.g., `operator.add` for lists).
  - **Persistence**: Use `SqliteSaver` for session persistence and "Human-in-the-loop" approval points.
  - **Time Travel**: Enable rewinding to previous states for debugging or user corrections.
