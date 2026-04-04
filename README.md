# ELITE V3: The Native sLM Intelligence Solution

> **"Beyond Scripting: Architecting High-Performance Local Intelligence with Rust and Flutter."**

ELITE V3는 Python 기반 오케스트레이션(LangGraph 등)의 런타임 오버헤드와 의존성 비대화를 극복하기 위해 설계된 **차세대 sLM(Small Language Model) 확장 솔루션**입니다. "UI는 Dart, 비즈니스 로직은 Rust"라는 아키텍처적 철학을 바탕으로, 국지적 인지 작업(Local Cognitive Tasks)을 위한 최적의 네이티브 환경을 제공합니다.

---

## 🏛️ 아키텍처 및 철학 (Architecture & Philosophy)

본 프로젝트는 단순한 도구의 집합을 넘어, 근대적 성공이 낳은 '위험사회'의 복잡성을 처리하기 위한 **성찰적 도구**로서 설계되었습니다. 기존의 스크립트 기반 AI 에이전트가 가진 구조적 취약성과 '위험의 아웃소싱' 현상을 배제하고, 시스템의 안정성과 성능을 최우선으로 하는 **Native-First** 접근법을 채택했습니다.

*   **Zero-Script Core**: 복잡한 에이전트 상태 머신을 Rust 네이티브 코드로 구현하여 실행 속도와 타입 안전성을 극대화했습니다. (LangGraph의 논리적 계승 및 탈피)
*   **Decentralized Intelligence**: 외부 API 의존성 없이 로컬 임베딩(MiniLM)과 GGUF 추론(Candle)을 단일 바이너리 내부에서 수행합니다.
*   **Asynchronous Streaming UX**: Flutter와 Rust 간의 고성능 비동기 통신(SSE)을 통해 지연 없는 실시간 인지 피드백과 동적 시각화(Visualization 2.0)를 제공합니다.

---

## 🚀 운영 매뉴얼 (Operational Manual)

### 🛠️ 사전 준비 사항 (Prerequisites)
1.  **Rust**: 최신 스테이블 버전 (1.74+)
2.  **Flutter SDK**: 3.10+ (Windows Desktop 지원 포함)
3.  **LLM 가중치**: `models/` 디렉토리에 다음 파일 확인
    *   `SmolLM2-1.7B-Instruct-Q4_K_M.gguf`
    *   `models/.cache/huggingface/tokenizer.json`

### 🏃 실행 및 테스트 절차
1.  **지식 마이그레이션**: 기존 마크다운 데이터를 네이티브 벡터 인덱스로 변환합니다.
    ```powershell
    cd elite_core
    cargo run --bin migrate
    ```
2.  **백엔드 서버 구동**: 에이전트 로직 및 로컬 추론 엔진을 실행합니다.
    ```powershell
    cd elite_core
    cargo run --bin elite_server
    ```
3.  **프론트엔드 UI 실행**: GUI 앱을 구동합니다.
    ```powershell
    cd elite_mobile_gui
    flutter run -d windows
    ```

---

## 🗺️ 프로젝트 로드맵 (Roadmap)

### Phase 1: 아키텍처 대전환 (Completed)
- [x] Legacy Python Backup 및 Rust Core 서버(axum) 구축.
- [x] Flutter Thin Client 기반 UI/BL 분리 개편.

### Phase 2: Rust BL & RAG (Completed)
- [x] Rust 기반 에이전트 상태 머신 및 고성능 RAG 엔진(`ndarray`) 구현.
- [x] Candle 기반 로컬 임베딩 및 GGUF 추론 엔진 통합.
- [x] 의미론적 마크다운 청킹 및 지식 마이그레이션 도구 완성.

### Phase 3: Inference & Performance Optimization (Active)
- [ ] GPU 가속(Vulkan/CUDA) 연동을 통한 추론 속도 극대화.
- [ ] GPU Buffer Orchestration을 통한 정교한 모델 스위칭 로직.

### Phase 4: High-Fidelity & Self-Refactoring (Planned)
- [ ] Strategic Self-Audit: 응답 품질 자가 검증 로직 고도화.
- [ ] Rust 네이티브 크롤러 기반 Web Search 통합.

---

## 🎯 현재 작업 현황 (Task Registry)

### 1. Presentation Layer (Flutter/Dart)
- [x] SSE/NDJSON 토큰 스트리밍 정밀화.
- [x] **Visualization 2.0**: 동적 인터랙티브 차트(Line, Bar, Pie) 구현.

### 2. Business Logic Layer (Rust / `elite_core`)
- [x] **Embedding Unification**: 로컬 MiniLM 임베딩 구현.
- [x] **Native Inference**: Advanced Sampling(Temp, Top-P)이 적용된 로컬 추론.

### 3. CI/CD & DevOps
- [x] Windows 빌드 안정성 확보 (Candle 기반).
- [x] **Rust Dockerization**: 멀티 스테이지 빌드 Dockerfile 구축.

---
**License**: Apache-2.0
**Maintainer**: ELITE Engineering Team
