# MAGI 시스템 사용자 및 운영 매뉴얼 (USER_MANUAL.md)

## 1. 개요 (Introduction)

본 문서는 MAGI(Multispectral Analysis & Global Integration) 시스템의 구조적 운영 절차와 오프라인 로컬 환경에서의 구동 방식을 다룹니다. MAGI는 외부의 중앙집중화된 거대 AI 클라우드 종속성에서 탈피하여, 사용자의 로컬 환경 내 독자적인 연산 자원을 바탕으로 복수의 특화 소형 모델(sLM)을 상호 적대적으로 조율하고 합의를 유도하는 지식 조율 플랫폼입니다. 본 플랫폼은 분석의 정밀도를 향상하고 실증적 근거 수준을 높이기 위해 설계되었습니다.

---

## 2. 구동 환경 및 사전 준비 (Getting Started)

### 2.1. 하드웨어 요구 사양
- **메모리(RAM/VRAM)**: 최소 **12GB** 이상의 여유 시스템 메모리가 요구됩니다. 특히 GPU 가속 연산을 할당할 경우, 2GB 이상의 VRAM을 탑재한 NVIDIA 외장 그래픽 카드(예: MX450 등)와 전용 CUDA 드라이버 환경이 권장됩니다.
- **CPU**: 최소 8코어 16스레드 급 프로세서(예: AMD Ryzen 7 5800U) 환경에서 동시 다중 유닛의 병렬 추론 연산 처리가 매끄럽게 보장됩니다.

### 2.2. 소프트웨어 빌드 및 배포 패키지 구성
1. 최신 빌드 바이너리 및 GUI 자원을 패키징하기 위해 PowerShell 터미널을 실행한 뒤, 저장소 루트에서 배포 패키징 스크립트([build_dist.ps1](file:///e:/llmassist/upload/scripts/build_dist.ps1))를 구동합니다.
   ```powershell
   ./scripts/build_dist.ps1
   ```
2. 생성된 실행용 통합 바이너리 및 라이브러리 파일들은 자동 생성되는 `dist/` 폴더 내에 배치됩니다.

### 2.3. 로컬 sLM 모델 배치 (GGUF 규격)
- 시스템이 구동을 정상적으로 완수하기 위해서는 사전 다운로드된 모델 파일(`.gguf`)이 `models/` 디렉토리에 정확히 위치해야 합니다.
- `models/` 내에 배치할 소형 언어 모델 목록은 아래와 같으며, 상세 매핑 설정은 프로젝트 루트의 [magi_config.ini](file:///e:/llmassist/upload/magi_config.ini)에서 변경 가능합니다:
  - **Phi-4** (Melchior - 초안 작성용)
  - **Gemma-3** (Balthasar - 윤리 및 비평용)
  - **DeepSeek-R1-Distill** (Casper - 고성능 논리 추론용)
  - **SmolLM2** (Artaban - 핵심 압축 요약용)
  - **DeepSeek-Coder** (Gushnasaph - 기술 및 명세 검증용)
  - **Qwen2.5-Math** (Kagba - 수치 검산용)

---

## 3. 시스템 기동 (Running the System)

본 시스템은 백엔드 엔진(Rust)과 모니터링 대시보드 및 채팅 인터페이스를 관장하는 프론트엔드(Flutter GUI)가 상호 연결되는 클라이언트-서버 구조로 작동합니다.

1. **원터치 통합 시작**: PowerShell을 열고 루트 디렉토리에서 아래 통합 기동 스크립트를 구동합니다.
   ```powershell
   ./scripts/start_magi.ps1
   ```
2. **시작 시나리오**:
   - `start_magi.ps1`이 실행되면, 먼저 백엔드 서비스인 `magi_core` 서버가 로컬 포트(`127.0.0.1:8000`)에 기동되며 필요한 포트 및 Sled 데이터베이스 환경의 가용성을 점검합니다.
   - 백엔드의 준비가 완료됨이 감지되면, 프론트엔드인 `magi_gui` 데스크톱 컴포넌트가 자동으로 부팅되어 연결됩니다.

---

## 4. 핵심 기능의 작동 원리 (Key Features)

### 4.1. 실시간 다층 합의 모니터링 (Adversarial Consensus Stream)
- 입력 필드에 학술적 혹은 다각적 분석이 필요한 복합 질문을 타이핑하고 전송합니다.
- 사용자는 화면 상에서 **Melchior**가 작성한 지식 초안에 대해 **Balthasar**의 윤리 비평, **Casper**의 논리 구조 검증, **Kagba**의 실증 검사 프로세스가 순차적 및 병렬적으로 가동되는 실시간 추론 스트림을 그래픽 인터페이스를 통해 직접 추적할 수 있습니다.

### 4.2. 모델 핫스왑 기능 (Model Hot-Swap)
- 대화 중간에 특정 전문 유닛의 추론용 모델을 교체하고자 할 경우, 화면 좌측의 사이드바 메뉴를 엽니다.
- **Model Settings** 탭에서 대상을 지정한 후, 새롭게 적용할 GGUF 파일을 로컬 스토리지 상에서 로드하면 백엔드 서버의 정지 및 재부팅 없이 실시간으로 Candle 가속 엔진에 마운트(Mount)가 완료됩니다.

### 4.3. 텔레메트리 대시보드 및 지식 추적
- 대화창 내 개별 메시지의 **Reasoning** 토글을 누르면, 가공되지 않은 DeepSeek-R1의 연쇄적 사고(Chain of Thought) 원시 로그가 노출되어 지능 조율의 투명성을 보장합니다.
- 우측의 보조 패널(Telemetry Dashboard)에서는 실시간 컨텍스트 잔여 토큰량, 쿼리 벡터화 시 계산된 코사인 유사도 점수 지표를 즉각 판독할 수 있습니다.

---

## 5. 장애 진단 및 문제 해결 (Troubleshooting)

- **OutOfMemory (OOM) 발생 및 시스템 느려짐**:
  - 16GB 이하 메모리 자원 환경에서 6개 유닛에 너무 크고 무거운 GGUF 파일(예: 8B 양자화 이상)이 동시 마킹될 경우 발생합니다.
  - 이 경우 [magi_config.ini](file:///e:/llmassist/upload/magi_config.ini)에서 활성 양자화 단계를 `Q4_K_M` 수준으로 경량화하거나, `max_active_models` 수치를 낮추어 RAM의 병목 임계치를 사전에 제어하십시오.
- **백엔드 통신 먹통 및 API 에러**:
  - 시스템 구동 도중 프론트 GUI가 백엔드와의 연결 해제 상태를 보고할 경우, [logs/magi_server.log](file:///e:/llmassist/upload/logs/magi_server.log) 파일의 Rust 디버그 추적 및 trace 로깅 내역을 파싱하여 세부 예외 상태를 검출하십시오.
