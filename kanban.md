# MAGI Project Kanban Board

## 🔴 Todo (할 일)

### 1. 마이그레이션 - 모델 및 엔진 (Migration - Core)
- [ ] **GGUF 모델 라이브러리 완비**: `magi_config.ini`에 지정된 모든 모델(Phi-4, Gemma-3, DeepSeek-R1 등)의 최신 GGUF 파일 다운로드 및 `models/` 배치.
- [ ] **엔진 벤치마크 및 최적화**: 로컬 런타임(Candle/Llama.cpp)에서의 추론 속도 측정 및 시스템 자원(VRAM/RAM) 최적 할당값 도출.
- [ ] **로컬 검색 사이드카 안정화**: `web_search_sidecar.py`의 Playwright 의존성 및 에지 케이스(Bot 차단 등) 대응 로직 강화.

### 2. 마이그레이션 - 관리 및 UI (Migration - Ops)
- [ ] **독립 실행 환경 전환**: Gemini-CLI 없이 `start_magi.ps1` 또는 `magi_gui` 단독 실행 시의 환경 설정(PATH, 라이브러리 경로) 자동화.
- [ ] **프로젝트 지식(Context) 문서화**: 현재까지 Gemini-CLI가 학습한 프로젝트 규칙, 페르소나 설정, 특이사항을 로컬 RAG용 지식 베이스로 이관.
- [ ] **Startup 등록 스크립트 수정**: 윈도우 시작 시 Gemini-CLI 대신 `magi_server`와 `magi_gui`를 바로 띄우도록 `register_startup.ps1` 업데이트.

### 3. 기능 확장 (Feature Backlog)
- [ ] **sLM 환각 제어 연구**: 소형 모델의 컨텍스트 윈도우 한계에 따른 환각 방지를 위한 네이티브 프롬프트 엔지니어링 강화.
- [ ] **PDF 리포트 스타일 고도화**: 출력 리포트의 폰트, 헤더, 페이지 번호 등 심미적 완성도 제고.

---

## 🟡 In Progress (진행 중)
- [ ] **Gemini-CLI 마이그레이션 설계**: 무료 티어 종료 대비 로컬 독립 시스템으로의 전환 계획 수립 및 검증.
- [ ] **작업 맥락 백업 및 칸반화**: 지식의 연속성 유지를 위한 `kanban.md` 구축 및 메모 작성.

---

## 🟢 Done (완료)
- [x] **멀티 에이전트 합의 엔진 구현**: 6개 유닛(Melchior, Balthasar 등) 협업 로직 완성.
- [x] **MemFlow 오케스트레이션**: 컨텍스트 프루닝 및 예측형 모델 로딩 레이어 도입.
- [x] **GUI 현대화 (Deep Forest 테마)**: Flutter 기반 Clean Architecture 및 UI 개편 완료.
- [x] **Clean Architecture 구조화**: 도메인/인프라/어댑터 레이어 분리.

---

## 📑 Appendix: 현재 작업 맥락 (Current Task Context)

### 1. 시스템 아키텍처 상태
- **백엔드(Rust)**: `magi_core`가 API 서버 역할을 하며, `ResourceManager`를 통해 로컬 GGUF 모델을 동적으로 로드함. 현재 Candle 엔진과 Llama.cpp 엔드포인트를 모두 지원하는 하이브리드 구조임.
- **프론트엔드(Flutter)**: `magi_gui`가 백엔드와 통신하며 실시간 추론 과정을 시각화함. Clean Architecture가 적용되어 모듈화가 잘 되어 있음.
- **오케스트레이션**: 6명의 '현자(Magi)' 유닛이 각각의 전문 분야(기술, 윤리, 논리 등)에 따라 답변 초안을 내고 합의하는 구조임.

### 2. 마이그레이션 핵심 동기
- **Gemini-CLI 무료 티어 종료 (9일 남음)**: 외부 API 및 클라우드 에이전트에 대한 의존성을 0으로 만들고, 5800U 기반 로컬 자원만을 사용하여 프로젝트를 지속 가능하게 만드는 것이 최우선 과제임.

### 3. 주요 기술적 메모
- **MemFlow**: sLM의 짧은 컨텍스트를 극복하기 위해 '의도 분석 -> 관련 컨텍스트 추출 -> 프루닝 -> 추론'의 파이프라인을 사용함.
- **VRAM 관리**: `ResourceManager`가 LRU 방식으로 최대 3개의 모델만 메모리에 올림. 마이그레이션 시 이 제한 수치를 시스템 RAM 용량에 맞춰 최적화해야 함.
- **검색**: 외부 API(Tavily) 의존성을 줄이기 위해 Python 사이드카(`web_search_sidecar.py`)를 통한 로컬 스크래핑 방식으로의 완전 전환이 필요함.

### 4. 미결 이슈 및 주의사항
- **모델 경로**: `magi_config.ini`와 실제 `models/` 폴더 내의 파일명이 일치해야 함.
- **환경 변수**: `.env` 파일에 API Key가 없더라도 시스템이 'Native Mode'로 정상 작동하도록 로직 체크가 필요함.
- **지식 이관**: Gemini-CLI가 그동안 대화를 통해 습득한 '학술적이고 분석적인 문체'와 '사회학적 통찰' 등의 페르소나를 `prompts/` 폴더의 시스템 프롬프트에 더 정교하게 녹여내야 함.

---
*Last Updated: 2026-06-08 (Gemini-CLI Migration Planning)*
