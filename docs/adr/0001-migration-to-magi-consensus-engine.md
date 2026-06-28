# ADR 0001: MAGI 합의 엔진 및 로컬 자원 기반 비대칭 오케스트레이션으로의 마이그레이션

## 1. Context (배경)

기존 시스템은 CPU 중심의 순차 추론 아키텍처(`proposal.md`)로 기획되었으며, Ryzen 5 3600 및 RX 580이라는 제한된 이기종 컴퓨팅 자원하에서 동작하도록 고안되었습니다. 그러나 2026년 6월 현재, 로컬 컴퓨팅 가용 자원이 AMD Ryzen 7 5800U (8 Cores, 16 Threads), 16GB RAM, NVIDIA GeForce MX450 (2GB VRAM) 환경으로 전환됨에 따라 다음과 같은 핵심 과제와 제약 사항이 도출되었습니다:

1. **로컬 자원 제약 하의 추론 성능 극대화**: 클라우드 API 의존성을 완전 배제하는 'Zero-Cloud Policy'에 따라, 단일 대형 모델(LLM)의 무거운 연산을 우회하고 소형 언어 모델(sLM)의 유기적 조합을 통한 창발적 지능을 확보해야 합니다.
2. **다중 에이전트 간의 비평 루프(Consensus Loop) 필요성**: 단일 추론의 한계인 환각 현상(Hallucination) 및 논리적 오류를 제어하기 위해 학술적, 기술적, 윤리적 관점을 상호 검증하는 다층적 비평 구조가 요구됩니다.
3. **VRAM 및 메모리 병목**: MX450의 2GB VRAM은 복수의 모델을 동시에 메모리에 상주시키기에 극도로 협소합니다. 이에 따라 동적인 모델 메모리 스왑 및 토큰 가지치기(MemFlow) 레이어의 도입이 불가피해졌습니다.

## 2. Decision (결정)

이에 본 프로젝트는 다음의 아키텍처적 전환을 결정하고 구현을 완료하였습니다:

1. **6개 전문 유닛(Magi Units)의 병렬 합의 프로토콜 도입**:
   - **Melchior (기술 아키텍처)**, **Balthasar (윤리 및 사회학)**, **Casper (논리적 추론/DeepSeek-R1)**, **Artaban (문맥 요약/SmolLM2)**, **Gushnasaph (코드 최적화)**, **Kagba (수치 및 실증 분석)**로 구성된 6개 유닛이 Draft-Critique-Consensus의 3단계 합의를 거치도록 설계하였습니다.
2. **Clean Architecture 계층 분리**:
   - 비즈니스 도메인의 순수성을 보존하는 Domain 계층, 비평 및 흐름을 조율하는 Application 계층, 입출력 변환을 담당하는 Interface Adapters 계층, Candle/Sled 등 구체 기술을 래핑하는 Infrastructure 계층으로 완벽히 모듈화하였습니다.
3. **MemFlow 메모리 오케스트레이터의 실구현**:
   - 4000자 초과 쿼리에 대한 동적 잘라내기(Truncation) 및 의미론적 가중치 기반 컨텍스트 가지치기(Semantic Context Pruning)를 수행하고, VRAM 용량 관리를 위해 LRU(Least Recently Used) 기반의 `ResourceManager`를 적용하여 모델 로딩 최적화를 달성하였습니다.

## 3. Status (상태)

**Accepted (승인 및 마이그레이션 완료)**

## 4. Consequences (결과)

### 긍정적 영향 (Consequences - Positive)
- **논리적 엄밀성 확보**: Balthasar의 비판적 사회학/젠더 연구 관점과 Casper의 논리적 디버깅 레이어가 결합되어 단순 생성형 답변이 아닌 다면적 성찰이 반영된 결과물을 산출합니다.
- **클라우드 종속성 제로**: 외부망 차단 상태에서도 5800U 하드웨어 스레드를 최대로 활용하여 독립적인 로컬 추론 가용성을 확보하였습니다.
- **유지보수성 향상**: Clean Architecture 패턴에 따라 Core 엔진의 비즈니스 로직 수정이 GUI(Flutter)나 인프라 드라이버(Candle)의 변화에 영향을 받지 않습니다.

### 부정적/지연 영향 (Consequences - Negative)
- **CPU 직렬 추론 병목**: 복수의 sLM 비평 과정에서 스레드 경쟁 및 CPU 컨텍스트 스위칭 오버헤드가 발생하여 Throughput이 다소 희생됩니다.
- **VRAM 물리적 한계**: MX450(2GB)에는 1~2개의 양자화 모델만 상주할 수 있으므로, 타 모델 호출 시 디스크 I/O 스왑 지연이 발생합니다. 이는 MemFlow의 예측 로딩(Predictive Loading) 및 LRU 반환 성능 튜닝으로 지속적 보완이 요구됩니다.
