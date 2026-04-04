# ELITE V3: Rust-Native Intelligence Roadmap

본 로드맵은 **"UI는 Dart, BL은 Rust"**라는 철학을 바탕으로, Python의 오버헤드를 완전히 제거하고 네이티브 수준의 성능을 지향하는 차세대 에이전트 구축 계획입니다.

## 💡 핵심 비전 (Core Vision)
- **Zero-Script Architecture**: 중재 계층인 Python을 제거하고 Rust 바이너리가 모든 비즈니스 로직을 직접 수행.
- **Native Inference Control**: LLM 추론 및 벡터 검색을 Rust 프로세스 내부에서 최적으로 관리.
- **Async Streaming UX**: Flutter와 Rust의 비동기 채널(mpsc)을 연동한 끊김 없는 실시간 스트리밍.

## 🗺️ 단계별 추진 계획 (Execution Phases)

### Phase 1: 아키텍처 대전환 (Completed)
- [x] **Legacy Python Backup**: 기존 LangGraph 코드를 `backups/legacy_python`으로 격리.
- [x] **Rust Core Scaffolding**: `axum` 및 `tokio` 기반의 고성능 API 서버 구축.
- [x] **Flutter Thin Client**: UI 로직과 BL을 분리하여 Rust 서버와 직접 통신하도록 개편.

### Phase 2: Rust Business Logic & RAG (Current)
- [x] **Rust Graph Engine**: LangGraph의 상태 머신 로직을 Rust 비동기 흐름으로 완벽 포팅.
- [x] **Rust-Native RAG**: `ndarray` 기반의 고성능 벡터 검색 엔진 구현.
- [ ] **Knowledge Migration**: 기존 PDF/JSON 지식 데이터를 Rust RAG 인덱스로 재구축.
- [ ] **Embedding Unification**: API 방식에서 Rust-Native 임베딩(Candle 등)으로 전환.

### Phase 3: Inference & Performance Optimization
- [x] **Modular OAI client**: Rust에서 외부 추론 엔진을 호출하는 안정적인 스트리밍 모듈 구현.
- [ ] **Native Llama-cpp Integration**: Windows 빌드 이슈 해결 및 Rust 내부에서 직접 GGUF 추론.
- [ ] **GPU Buffer Orchestration**: RX 580 VRAM을 직접 관리하는 정교한 모델 스위칭 로직.

### Phase 4: High-Fidelity & Self-Refactoring
- [ ] **Advanced Visualization**: Flutter를 활용한 고차원 데이터 시각화 라이브러리 확장.
- [ ] **Strategic Self-Audit**: Rust BL이 스스로 응답 품질을 검증하고 경로를 수정하는 7-Point Audit 로직 고도화.
- [ ] **Web Search Rust Integration**: Python 기반 검색 모니터를 Rust 네이티브 크롤러로 대체.

---
**Last Updated:** 2026-04-04
**Core Stack:** Rust (axum, tokio, ndarray) / Dart (Flutter, Riverpod)
**Status:** **Rust BL Core Migration Active.**
