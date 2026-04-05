# llamelphi: High-Fidelity sLM Orchestration Framework

**llamelphi**는 소규모 언어 모델(sLM)의 한계를 극복하고, 복잡한 지시에 대해 고품질의 답변을 생성하기 위해 설계된 **MAGI 시스템 기반의 Peer-Review 오케스트레이션 솔루션**입니다. "UI는 Dart, 비즈니스 로직은 Rust"라는 아키텍처를 통해 로컬 환경에서 최적의 추론 성능과 다각도 품질 검증 루프를 제공합니다.

---

## 💡 MAGI 시스템 (The MAGI System)

단일 모델의 추론에 의존하는 대신, 특화된 역할을 가진 세 명의 현자(시스템 노드)가 **제안-비판-수정-합성**의 협력 과정을 거치게 함으로써 거대 모델(LLM) 수준의 정교한 답변 품질을 지향합니다.

*   **MAGI Peer-Review Loop**: 생성된 초안에 대해 복수의 리뷰어 모델이 스타일, 논리, 사실 관계를 상호 검증합니다.
*   **Ensemble Orchestration**: 추론(Reasoning), 초안(Drafting), 검토(Reviewing), 합성(Synthesis)으로 역할을 분담하여 복잡한 지시 수행 능력을 극대화합니다.
*   **Rust-Native Core**: MAGI 시스템의 모든 상태 전환 로직을 Rust 네이티브 코드로 구현하여 지연 시간을 최소화했습니다.

---

## 📐 시스템 설계 (System Design)

### 1. MAGI 앙상블 아키텍처
각 노드는 MAGI 오케스트레이터에 의해 독립적으로 제어됩니다.
*   **Strategic Reasoner (Melchior)**: 지시사항을 분석하고 사고 사슬(CoT) 및 논리 맵을 구성합니다.
*   **Contextual Drafter (Balthasar)**: 논리 맵을 바탕으로 상세 답변 초안을 작성합니다.
*   **Multi-Perspective Reviewers (Caspar)**: 초안의 가독성, 전문성, 정확성을 독립적으로 검토합니다.
*   **Final Synthesizer**: 리뷰 피드백을 반영하여 최종 결과물을 정제합니다.

### 2. 하이브리드 리소스 최적화
*   **VRAM Resident**: 임베딩 모델(MiniLM)을 상주시켜 즉각적인 의미론적 검색을 지원합니다.
*   **Dynamic Loading**: 대용량 모델(GGUF)은 `Candle` 프레임워크를 통해 추론 시점에만 동적으로 로드됩니다.

---

## 🚀 운영 가이드 (Operational Guide)

### 🛠️ 사전 준비 (Prerequisites)
*   **Rust**: 1.74+
*   **Flutter**: 3.10+
*   **Models**: `models/` 디렉토리에 GGUF 형식 가중치 및 토크나이저 설정 필요.

### 🏃 실행 절차
1.  **지식 베이스 구축**:
    ```powershell
    cd elite_core
    cargo run --bin migrate
    ```
2.  **MAGI 서버 실행**:
    ```powershell
    cd elite_core
    cargo run --bin llamelphi_server
    ```
3.  **UI 터미널 실행**:
    ```powershell
    cd elite_mobile_gui
    flutter run -d windows
    ```

---
**License**: Apache-2.0
**Maintainer**: llamelphi Project Team

