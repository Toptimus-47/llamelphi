# MAGI Project Architecture & Operational Blueprint (2026)

## 1. System Overview
MAGI는 5800U 로컬 자원을 최적화하여 사용하는 **6-Agent Consensus System**입니다. 클라우드 의존성을 제거하고 완전한 오프라인 작동을 목표로 합니다.

## 2. Core Components (The "Magi" Units)
- **Melchior (Technical)**: 시스템 아키텍처 및 구현 가능성 검토.
- **Balthasar (Ethics/Social)**: 젠더 연구, 비판적 사회학 기반의 윤리적 가이드라인 제시.
- **Casper (Logic/Reasoning)**: DeepSeek-R1 기반의 논리적 추론 및 단계별 분석.
- **Artaban (Summary/Context)**: SmolLM2를 사용한 문맥 압축 및 핵심 요약.
- **Gushnasaph (Coding/Spec)**: Rust/Flutter 코드 최적화 및 기술 명세 작성.
- **Kagba (Math/Analysis)**: 수치 데이터 분석 및 수식 검증.

## 3. Technology Stack
- **Inference Engine**: `magi_core` (Rust) 기반의 Candle & Llama.cpp 하이브리드 엔진.
- **Frontend**: `magi_gui` (Flutter) 기반의 다중 패널 스트리밍 UI.
- **Orchestration (MemFlow)**: 
    - **Intent Analysis**: 사용자 의도 파악.
    - **Context Pruning**: sLM 한계 극복을 위한 동적 토큰 가지치기.
    - **Predictive Loading**: 다음 발화에 필요한 모델을 미리 VRAM에 로드 (LRU 방식).

## 4. Maintenance & Operations
### 실행 (Execution)
- **Primary**: `scripts/start_magi.ps1` (Backend + Frontend 통합 실행)
- **Diagnostic**: `scripts/run_production_test.ps1` (시스템 무결성 점검)

### 설정 (Configuration)
- **Model Mapping**: `magi_config.ini`에서 각 유닛의 GGUF 파일 경로 및 백엔드 타입 지정.
- **Knowledge Ingestion**: `vector_db/` 폴더에 새 문서를 추가하면 RAG 엔진이 자동 인식.

## 5. Knowledge Continuity (Migration Note)
- **문체 및 어조**: 학술적, 비판적, 성찰적 어조를 유지할 것. (참조: `GEMINI.md`)
- **보안 원칙**: PII 마스킹 레이어를 거치지 않은 데이터는 절대 외부로 유출하지 말 것.
- **로컬 우선**: 모든 기능은 인터넷 연결 없이도 작동해야 함 (`web_search_sidecar.py` 활용).

---
*Created for Gemini-CLI Migration (2026-06-08)*
