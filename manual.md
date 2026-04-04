# ELITE V3 Operational Manual: Rust-Native Intelligence

본 매뉴얼은 **ELITE V3** 시스템의 아키텍처 이해와 원활한 구동을 위한 기술 지침서입니다. 본 시스템은 Python의 런타임 오버헤드를 배제하고, **순수 Rust(Core Logic)와 Dart(Flutter UI)**의 결합을 통해 고성능 국지적 지능(Local Intelligence)을 구현하는 것을 목적으로 합니다.

---

## 🏗️ 시스템 아키텍처 (System Architecture)

*   **Presentation Layer (Flutter/Dart)**: 비동기 스트리밍(SSE) 및 동적 데이터 시각화(Visualization 2.0)를 담당하는 씬 클라이언트(Thin Client).
*   **Business Logic Layer (Rust / `elite_core`)**: `axum` 기반의 고성능 서버로, 에이전트 상태 머신, RAG 엔진, 로컬 임베딩 및 GGUF 추론을 Rust 네이티브 환경에서 직접 수행.
*   **Data Layer**: 벡터화된 지식 베이스(`knowledge_base.json`)와 GGUF 형식의 로컬 LLM 가중치.

---

## 🛠️ 사전 준비 사항 (Prerequisites)

1.  **Rust**: 최신 스테이블 버전 (1.74+)
2.  **Flutter SDK**: 3.10+ (Windows Desktop 지원 포함)
3.  **C++ Build Tools**: MSVC (Windows) 또는 GCC/Clang (Linux)
4.  **LLM 가중치**: `models/` 디렉토리에 다음 파일 확인
    *   `SmolLM2-1.7B-Instruct-Q4_K_M.gguf`
    *   `models/.cache/huggingface/tokenizer.json` (토크나이저 설정 파일)

---

## 🚀 실행 및 테스트 절차 (Execution Flow)

### 1. 지식 마이그레이션 (Knowledge Migration)
기존 마크다운 지식 데이터를 Rust 네이티브 벡터 인덱스로 변환합니다. 새로운 `.md` 파일을 `knowledge_*.md` 패턴으로 추가한 경우 반드시 실행해야 합니다.

```powershell
cd elite_core
cargo run --bin migrate
```
*   **결과**: `vector_db/knowledge_base.json` 생성. 로컬 MiniLM 모델을 통해 의미론적 임베딩이 수행됩니다.

### 2. 백엔드 서버 구동 (Running Rust Core)
에이전트 로직과 추론 엔진을 담당하는 서버를 실행합니다.

```powershell
cd elite_core
cargo run --bin elite_server
```
*   **서버 주소**: `http://localhost:8000`
*   **기능**: 로컬 GGUF 모델 자동 감지, RAG 인덱스 로드, 하이브리드 추론 엔진 활성화.

### 3. 프론트엔드 UI 실행 (Running Flutter UI)
현대적인 스트리밍 UX와 차트 시각화를 제공하는 GUI 앱을 구동합니다.

```powershell
cd elite_mobile_gui
flutter run -d windows
```

---

## 🧪 기능 테스트 (Testing & Validation)

### CLI 기반 스트림 테스트
UI 없이 서버의 응답성을 즉시 확인하려면 다음 `curl` 명령어를 사용하십시오.

```powershell
curl -X POST http://localhost:8000/chat/stream `
     -H "Content-Type: application/json" `
     -d '{"query": "울리히 벡의 성찰적 근대화에 대해 설명해줘", "session_id": "manual-test"}'
```

### 시각화(Visualization) 테스트
대화 중 "최근 사회적 위험 지표의 변화를 차트로 보여줘"와 같이 데이터가 포함된 질문을 던지면, 하단에 동적 차트(Line, Bar, Pie)가 애니메이션과 함께 렌더링됩니다.

---

## 🐳 컨테이너화 배포 (Dockerization)
배포 편의성을 위해 멀티 스테이지 빌드 환경을 지원합니다.

```powershell
cd elite_core
docker build -t elite-v3-core .
docker run -p 8000:8000 elite-v3-core
```

---
**Last Updated**: 2026-04-04
**Project Maintainer**: ELITE V3 Engineering Team
