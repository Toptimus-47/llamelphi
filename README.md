# ELITE V3: High-Fidelity sLM Orchestration Framework

ELITE V3는 소규모 언어 모델(sLM)의 한계를 극복하고, 복잡한 지시에 대해 고품질의 답변을 생성하기 위해 설계된 **Peer-Review 기반 오케스트레이션 솔루션**입니다. "UI는 Dart, 비즈니스 로직은 Rust"라는 아키텍처를 통해 로컬 환경에서 최적의 추론 성능과 품질 검증 루프를 제공합니다.

---

## 💡 핵심 개념 (Core Concept)

단일 모델의 추론에 의존하는 대신, 특화된 역할을 가진 여러 sLM들이 **제안-비판-수정-합성**의 협력 과정을 거치게 함으로써 거대 모델(LLM) 수준의 정교한 답변 품질을 지향합니다.

*   **Peer-Review Loop**: 생성된 초안에 대해 복수의 리뷰어 모델이 스타일, 논리, 사실 관계를 상호 검증합니다.
*   **Ensemble Orchestration**: 추론(Reasoning), 초안(Drafting), 검토(Reviewing), 합성(Synthesis)으로 역할을 분담하여 복잡한 지시 수행 능력을 극대화합니다.
*   **High-Performance Native Core**: 이 모든 복잡한 상태 전환 로직을 Rust 네이티브 코드로 구현하여 지연 시간을 최소화했습니다.

---

## 📐 시스템 설계 (System Design)

### 1. 모델 앙상블 아키텍처 (Model Ensemble)
각 노드는 독립적인 추론 컨텍스트를 가지며, 에이전트 상태 머신에 의해 제어됩니다.
*   **Strategic Reasoner**: 지시사항을 분석하고 사고 사슬(CoT) 및 논리 맵을 구성합니다.
*   **Contextual Drafter**: 논리 맵을 바탕으로 상세 답변 초안을 작성합니다.
*   **Multi-Perspective Reviewers**: 초안의 가독성, 전문성, 정확성을 독립적으로 검토합니다.
*   **Final Synthesizer**: 리뷰 피드백을 반영하여 최종 결과물을 정제합니다.

### 2. 하이브리드 리소스 최적화 (Resource Management)
제한된 컴퓨팅 자원에서도 안정적인 멀티 모델 구동이 가능하도록 설계되었습니다.
*   **VRAM Resident**: 임베딩 및 라우팅 모델을 상주시켜 즉각적인 의미론적 검색을 지원합니다.
*   **Dynamic Loading**: 대용량 모델(GGUF)은 `Candle` 프레임워크를 통해 추론 시점에만 메모리에 로드되어 리소스 충돌을 방지합니다.

---

## 🚀 운영 가이드 (Operational Guide)

### 🛠️ 사전 준비 (Prerequisites)
*   **Rust**: 1.74+ (Backend Core)
*   **Flutter**: 3.10+ (Cross-platform UI)
*   **Models**: `models/` 디렉토리에 GGUF 형식의 모델 가중치 및 토크나이저 설정 필요.

### 🏃 실행 절차
1.  **지식 베이스 구축**:
    ```powershell
    cd elite_core
    cargo run --bin migrate
    ```
2.  **오케스트레이션 서버 실행**:
    ```powershell
    cd elite_core
    cargo run --bin elite_server
    ```
3.  **UI 터미널 실행**:
    ```powershell
    cd elite_mobile_gui
    flutter run -d windows
    ```

---

## 🗺️ 로드맵 (Roadmap)

### Completed
- [x] Rust 기반 에이전트 상태 머신 및 오케스트레이션 엔진 구현.
- [x] 순수 Rust 기반 로컬 임베딩 및 GGUF 추론(Candle) 통합.
- [x] SSE 기반 실시간 토큰 스트리밍 및 동적 시각화(Visualization 2.0).

### Planned
- [ ] GPU 가속(Vulkan/CUDA) 정밀 연동.
- [ ] Self-Audit: 답변 품질 자가 측정 및 자동 재시도 루프 고도화.
- [ ] 네이티브 크롤러 기반의 실시간 정보 보강(RAG) 엔진 통합.

---
**License**: Apache-2.0
**Maintainer**: ELITE Engineering Team
