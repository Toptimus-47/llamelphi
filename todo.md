# ELITE Project TODO List

## ✅ 완료된 작업 (Completed)
- [x] **학술적 근거 기반 비서 설계**: ASD/PDD, 신체형 장애 대응 프롬프팅 전략 수립.
- [x] **강력한 개인정보 격리 (PII Isolation)**: `knowledge_*.md` 추적 해제 및 무결성 점검 스크립트 도입.
- [x] **로컬 RAG 엔진 고도화**: 메타데이터 기반 정교한 검색 로직 구현.
- [x] **멀티모달 통합 (V1)**: `Vision` 노드를 통한 `Image-to-Text` 분석 및 텍스트 리포팅 파이프라인 구축.
- [x] **실시간 지식 최신화 (V1)**: `WebSearch` 노드 추가 및 동적 라우팅 로직 통합.
- [x] **Multi-pass Refinement**: `proposal.md` 철학에 기반한 **Draft-Critique-Revision** 루프 구현.
- [x] **n8n 테스트 엔진 인프라**: 로컬 Docker 기반 n8n 서버 설정 파일(`docker-compose.yml`) 준비.
- [x] **로컬 Vision 모델 네이티브 통합**: `native_inference.rs`에서 Candle 기반 CLIP/LLaVA 직접 추론 구조 구현. (2026-04-15)
- [x] **실제 검색 API 연동**: `WebSearch` 노드에 Tavily API 실구현 및 환경변수 처리. (2026-04-15)
- [x] **성능 최적화**: `Cargo.toml`에 AVX-512/CUDA 가속 피처 도입. (2026-04-15)
- [x] **사용자 피드백 루프**: `feedback.jsonl` 기반 응답 품질 데이터 축적 엔드포인트 구현. (2026-04-15)
- [x] **각 Unit test & Acceptance test**: n8n용 테스트 시나리오 v1 작성 완료. (2026-04-15)
- [x] **보안 감사 (Security Audit)**: PII(이메일, 전화번호, 주민번호) 자동 마스킹 레이어 적용. (2026-04-15)
- [x] **모바일 GUI 고도화**: Flutter `fl_chart` 기반 실시간 시각화 엔진 연동 완료. (2026-04-15)
- [x] **강화 학습 기반 랭킹 (RL-based Ranking)**: 피드백 점수 기반 RAG 문서 가중치 자동 조정 인프라 구축. (2026-04-15)
- [x] **멀티 에이전트 협업 고도화**: 법률/의학 전문 에이전트 노드(`Specialist`) 및 동적 라우팅 구현. (2026-04-15)
- [x] **On-device Fine-tuning**: 피드백 기반 학습 데이터셋 생성 파이프라인(`train_prepare`) 구축. (2026-04-15)
- [x] **인프라 자동화 및 가상화 준비**: Docker Desktop 설치 및 WSL2 기반 가상화 엔진 설정 완료. (2026-04-16)
- [x] **보안 강화 및 무결성 점검**: `.gitignore` 최적화 및 민감 데이터(세션 로그, 프롬프트) 유출 방지 레이어 강화. (2026-04-16)
- [x] **코어 엔진 릴리즈 빌드**: `elite_core` 최적화 빌드(`--release`) 및 툴체인 경로 정상화. (2026-04-16)
- [x] **네이티브 아키텍처 전환**: BIOS 가상화 제약 극복을 위한 Docker 탈피 및 Rust/Node.js 원시 런타임 기반 통합 환경 구축 완료. (2026-04-23)
- [x] **6개 모델 병렬 협업 시스템 구현**: SDD.md 명세에 따른 6개 전문 유닛(Melchior, Balthasar, Casper, Artaban, Gushnasaph, Kagba)의 병렬 초안 생성 및 합의 로직 완성. (2026-04-29)
- [x] **프로덕션 패키징**: `dist/` 폴더 내 바이너리 및 모델 파일 통합 배치 완료. (2026-04-29)
- [x] **GUI UI/UX 현대화**: 2020년대 스타일의 Zinc-950 다크 테마, 멀티 판넬 레이아웃, 및 세련된 메시지 버블 디자인으로 전면 개편 완료. (2026-05-03)
- [x] **모델 로딩 최적화**: LRU 기반 `ResourceManager` 도입으로 메모리 부족 시 자동 모델 언로드 로직 구현. (2026-05-04)
- [x] **다국어 지원**: GUI 및 시스템 메시지의 다국어(KOR/ENG) 토글 기능 및 LocaleProvider 구현 완료. (2026-05-04)
- [x] **Graceful Shutdown**: FE 종료 시 BE가 순차적으로 정리되는 시스템 종료 프로토콜 구현 완료. (2026-05-04)
- [x] **FE 현대화 및 모듈화**: `main.dart`의 거대 구조를 `core`, `screens`, `widgets`, `services` 계층으로 완전 분리 및 Clean Architecture 적용. (2026-05-12)
- [x] **디자인 시스템 고도화**: 눈의 피로를 최소화하는 'Deep Forest' (명도 8, 채도 2) 그린 테마 및 에메랄드 포인트 컬러 시스템 구축. (2026-05-12)
- [x] **FE 구현 가이드라인 수립**: `FE_Implementation.md` 작성을 통해 현대적 AI UI 분석 기반의 개발 표준 확립. (2026-05-12)
- [x] **Interface Adapters 레이어 구조화**: `main.rs`에 집중된 API 엔드포인트 및 DTO 변환 로직을 `magi_core/src/adapters` 계층으로 분리하여 Clean Architecture의 엄밀성 강화. (2026-05-16)
- [x] **합의 엔진 최적화**: `consensus_engine.rs`의 거대 로직을 Draft/Critique/Consensus 서브 모듈로 분산하여 가독성 및 유지보수성 개선. (2026-05-16)
- [x] **Dual-Pane 레이아웃 고도화**: 우측 패널에 RAG 출처 문서(PDF) 미리보기 및 핵심 데이터 요약 뷰어 구현 완료. (2026-05-13)
- [x] **Streaming Markdown 강화**: LaTeX 수식 렌더링(`flutter_math_fork`) 및 코드 하이라이팅 플러그인 연동 최적화 완료. (2026-05-13)
- [x] **2024-2026 sLM 연구 지식 베이스 구축**: 최신 sLM 논문 15편의 메타데이터 및 초록 추출 완료. (2026-05-17)
- [x] **메모리 오케스트레이션(MemFlow) 구현**: 2026년 연구 동향(arXiv:2601.04512)을 반영하여 sLM의 컨텍스트 윈도우 한계를 극복하는 의도 기반 컨텍스트 가지치기(Pruning) 및 예측형 모델 로딩(Predictive Loading) 레이어 도입 완료. (2026-05-18)
- [x] **실제 모델 기반 합의 엔진 및 MemFlow 검증**: Phi-4, Gemma-3, SmolLM2 등 GGUF 모델을 로드하여 Full Orchestration 루프(Intent -> Pruning -> Consensus) 테스트 수행. 대규모 쿼리에 대한 컨텍스트 오버플로 해결 완료. (2026-05-18)
- [x] **GUI 통합 테스트 및 Transparency UX 완성**: 실제 모델 추론 결과를 Flutter GUI에 실시간 스트리밍으로 연동하고, 합의 과정을 시각적으로 노출 (Protocol 정합성 고도화 완료). (2026-05-18)

## 📅 향후 과제 (Backlog)
- [ ] **sLM 실제 모델 추론 품질 고도화**: 소형 모델의 긴 컨텍스트 처리 시 발생하는 환각 현상 제어 및 프롬프트 튜닝 연구.
- [ ] **다중 모델 병렬 로딩 최적화**: `ResourceManager`를 고도화하여 6개 유닛이 서로 다른 모델 파일을 사용할 때의 VRAM 할당 및 스왑 로직 정밀화. (CPU 추론 속도 병목 확인 및 GPU 가속 검토 필요)

---
*마지막 업데이트: 2026-05-18 (MemFlow 구현 및 실전 모델 통합 테스트 완료)*
