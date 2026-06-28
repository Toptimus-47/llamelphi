# llamelphi: MAGI Multi-Agent Orchestration

**llamelphi** is a high-fidelity research platform for localized intelligence, powered by the **MAGI** (Multispectral Analysis & Global Integration) orchestration engine. It implements a unique adversarial consensus protocol across multiple local language models to deliver logically rigorous and empirically grounded synthesis.

---

## 🌟 Key Features
- **Adversarial Consensus**: 6 specialized units (Melchior, Balthasar, etc.) engage in a multi-pass drafting and critique loop.
- **Local-First / Zero-Exposure**: All computation occurs on your machine. Sensitive data is masked and never leaves the local environment.
- **Transparent Intelligence**: Real-time visualization of internal reasoning logs, search strategies, and adversarial audits.
- **Industrial Standalone Core**: High-performance Rust backend with a modern Flutter GUI, bundled for native Windows execution.

---

## 📂 Project Navigation
To understand the system deeply, please refer to the following specialized documents:

| Document | Purpose |
| :--- | :--- |
| **[GEMINI.md](./GEMINI.md)** | **Core Mandates**: Foundations, Clean Architecture rules, and team-shared conventions. |
| **[SDD.md](./SDD.md)** | **System Design**: Exhaustive technical specification, DB schemas, and protocol details. |
| **[DEVELOPMENT.md](./DEVELOPMENT.md)** | **Engineering Standard**: Coding styles, Git workflows, and local CI/CD setup. |
| **[USER_MANUAL.md](./USER_MANUAL.md)** | **Usage Guide**: Installation, model setup, and feature walkthrough. |

---

## 🚀 Quick Start (Production)
1. Ensure you have **12GB+ RAM/VRAM** available.
2. Navigate to the `dist/` directory.
3. Run `.\launch_magi.bat`.
4. (Optional) Use the **Model Hot-Swap** feature in the sidebar to load your preferred GGUF models.

---

## 🛠️ Technical Stack
- **Backend**: Rust (Axum, Candle, Sled)
- **Frontend**: Flutter (Riverpod, Material 3 Industrial Theme)
- **Database**: Document DB (BSON/Sled) & Vector DB (Semantic Recall)
- **Models**: GGUF compatible sLMs (Phi-4, DeepSeek, SmolLM2, etc.)

---
**License**: Apache-2.0
**Repository**: [https://github.com/Toptimus-47/llamelphi](https://github.com/Toptimus-47/llamelphi)
