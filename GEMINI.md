# MAGI (Multispectral Analysis & Global Integration) - Project Mandates

This project follows the **Clean Architecture** principles and **SDG (Software Development Guideline)** standards. All development must adhere to the rules defined below.

## 1. Architectural Integrity (Clean Architecture)
The project is divided into four distinct layers:
- **Domain Layer (`magi_core/src/domain/`):** Pure business logic and entities. No external dependencies.
- **Application Layer (`magi_core/src/application/`):** Use cases and orchestration logic (`Orchestrator`).
- **Interface Adapters (`magi_core/src/infrastructure/`):** Translation between domain and external world (API, SSE, LLM Gateways).
- **Infrastructure Layer (`magi_core/src/infrastructure/`):** Concrete technical implementations (Candle inference, Vector DB).

**Constraint:** Infrastructure changes MUST NOT affect the Domain layer. Use Traits for dependency inversion.

## 2. Multi-Agent Consensus Protocol
The core reasoning follows a 3-step consensus algorithm:
1. **Drafting:** Parallel synthesis by 6 specialized units (Melchior, Balthasar, Casper, Artaban, Gushnasaph, Kagba).
2. **Reviewing:** Cross-unit evaluation of logic, technical accuracy, and ethics.
3. **Consensus:** Final harmonization by the Orchestrator to produce a single authoritative report.

## 3. Security & Privacy (Zero-Exposure)
- **PII Masking:** All inputs must be masked before processing. All outputs must be masked before returning to the user.
- **Local-First:** All computation is performed locally. No user data (except search queries) should leave the machine.
- **Ignore Patterns:** Knowledge bases, session history, and logs are excluded from version control to prevent data leakage.

## 4. Engineering Standards
- **Language:** Rust (Backend), Flutter/Dart (GUI).
- **Style:** Analytical, formal, and precise technical tone.
- **Error Handling:** Use `thiserror` and `anyhow`. Implement self-healing loops for inference failures.
- **Logging:** Use `tracing`. Level TRACE is preferred for debugging complex orchestration.

## 5. Development Workflow
- **Tests:** Maintain 80% coverage for Domain/Application layers. 100% for PII masking.
- **Commits:** Follow Conventional Commits (`feat:`, `fix:`, `refactor:`, `docs:`).
- **Documentation:** Major design decisions must be recorded in ADRs.

## 6. Local Autonomy & Knowledge Continuity (Post-Migration)
- **Zero-Cloud Policy**: 2026년 6월 이후 모든 추론 및 관리는 로컬 자원(5800U)으로 한정한다.
- **Documentation Over Memory**: 모든 핵심 아키텍처 결정은 `docs/ARCHITECTURE_BLUEPRINT.md`에 기록하며, AI의 단기 메모리보다 구조적 문서를 우선 신뢰한다.
- **Persona Preservation**: 에이전트의 '학술적/비판적' 페르소나는 `prompts/` 내의 시스템 프롬프트 파일을 통해 영구 보존한다.
- **Operational Integrity**: 실행 환경의 변화(Gemini-CLI 종료)에도 불구하고 `scripts/` 내의 통합 런처를 통해 시스템 가용성을 100% 유지한다.
