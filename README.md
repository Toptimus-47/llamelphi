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

## 📐 시스템 설계 (System Design)

본 시스템은 제한된 자원(Consumer-Grade Hardware) 환경에서도 최적의 추론 성능을 발휘하도록 설계되었습니다.

### 1. 하이브리드 메모리 전략 (Hybrid Memory Strategy)
*   **GPU (VRAM):** 의미론적 라우팅 및 RAG 벡터 검색을 위한 임베딩 모델(MiniLM)을 상주시켜 지연 시간을 최소화합니다.
*   **CPU (RAM):** 고용량 LLM(GGUF)을 효율적으로 로드하고 실행하며, `Candle` 프레임워크를 통해 메모리 점유를 정교하게 제어합니다.

### 2. 모델 앙상블 구조 (Model Ensemble)
인지 작업을 세분화하여 각 분야에 특화된 sLM들을 오케스트레이션합니다.
*   **Reasoning**: 논리 구조 및 CoT(Chain-of-Thought) 생성.
*   **Drafting**: 한국어 뉘앙스를 반영한 초안 작성.
*   **Reviewing**: 스타일, 의도, 사실 관계에 대한 다각도 검토.
*   **Synthesis**: 최종 답변 합성 및 정교화.

---

## 🚀 운영 매뉴얼 (Operational Manual)

### 🛠️ 사전 준비 사항 (Prerequisites)
1.  **Rust**: 최신 스테이블 버전 (1.74+)
2.  **Flutter SDK**: 3.10+ (Windows Desktop 지원 포함)
3.  **LLM 가중치**: `models/` 디렉토리에 다음 파일 확인
    *   `SmolLM2-1.7B-Instruct-Q4_K_M.gguf`
    *   `models/.cache/huggingface/tokenizer.json`

### 🏃 실행 및 테스트 절차
1.  **지식 마이그레이션**: `cd elite_core; cargo run --bin migrate`
2.  **백엔드 서버 구동**: `cd elite_core; cargo run --bin elite_server`
3.  **프론트엔드 UI 실행**: `cd elite_mobile_gui; flutter run -d windows`

---

## 🗺️ 프로젝트 로드맵 (Roadmap)

### Phase 2: Rust BL & RAG (Completed)
- [x] Rust 기반 에이전트 상태 머신 및 고성능 RAG 엔진(`ndarray`) 구현.
- [x] Candle 기반 로컬 임베딩 및 GGUF 추론 엔진 통합.
- [x] 의미론적 마크다운 청킹 및 지식 마이그레이션 도구 완성.

### Phase 3: Inference & Performance Optimization (Active)
- [ ] GPU 가속(Vulkan/CUDA) 연동을 통한 추론 속도 극대화.
- [ ] GPU Buffer Orchestration을 통한 정교한 모델 스위칭 로직.

---

## 🎯 현재 작업 현황 (Task Registry)

- [x] **Visualization 2.0**: 동적 인터랙티브 차트(Line, Bar, Pie) 구현.
- [x] **Native Inference**: Advanced Sampling이 적용된 로컬 추론.
- [x] **Rust Dockerization**: 멀티 스테이지 빌드 Dockerfile 구축.

---
**License**: Apache-2.0
**Maintainer**: ELITE Engineering Team
