# Software Design Document: SLM-based Multi-Modal Agent (MX450 Optimized)

- **Version:** 2.0
- **Date:** 2026-03-12
- **Author:** Gemini CLI
- **Status:** Active Development

---

## 1. Introduction

This document describes the architecture of a high-performance AI agent designed to run on low-spec hardware (specifically a laptop with NVIDIA MX450 2GB VRAM and 16GB RAM). By orchestrating an ensemble of 7 Small Language Models (SLMs) using a hybrid memory strategy, the system aims to achieve reasoning and generation capabilities comparable to larger models.

## 2. Goals and Objectives

-   **High Performance on Low Specs:** Maximize utility of 2GB VRAM and 16GB RAM.
-   **Specialized Roles:** Assign distinct cognitive tasks (Reasoning, Drafting, Reviewing, Synthesizing) to specialized SLMs.
-   **Strict Resource Management:** Implement a "Load-Infer-Unload" cycle for LLMs to prevent OOM (Out of Memory) errors, while keeping lightweight embeddings resident in VRAM.
-   **Multi-Modal Capability:** Integreate Vision capabilities (Moondream2) for image understanding.

## 3. System Architecture

The system uses a **Hybrid Memory Strategy**:
-   **GPU (VRAM 2GB):** Permanently hosts the Embedding Model (BGE-M3-Small) for fast routing and RAG. Loads Vision Model (Moondream2) on-demand.
-   **CPU (RAM 16GB):** Sequentially loads and executes LLMs. Uses `llama.cpp` with `mmap` for efficient I/O.
-   **Orchestration:** LangGraph manages the state and workflow execution.

### 3.1. Model Ensemble (The 7 Models)

| Role | Model | Filename (GGUF) | Function |
| :--- | :--- | :--- | :--- |
| **Reasoning** | DeepSeek-R1-1.5B | `DeepSeek-R1-Distill-Qwen-1.5B-Q4_K_M.gguf` | Generates Chain-of-Thought (CoT) and logic maps. |
| **Orchestrator 1** | Nanbeige-4.1-3B | `Nanbeige4.1-3B.Q4_K_M.gguf` | Drafts initial responses, capable of Korean nuances. |
| **Orchestrator 2** | Phi-4-mini | `microsoft_Phi-4-mini-instruct-Q4_K_M.gguf` | Synthesizes final answers from drafts and reviews. |
| **Reviewer 1** | Gemma-3-4B | `google_gemma-3-4b-it-Q4_K_M.gguf` | Reviews for style, intent, and readability. |
| **Reviewer 2** | SmolLM2-1.7B | `SmolLM2-1.7B-Instruct-Q4_K_M.gguf` | Checks facts and summarizes key points. |
| **Vision** | Moondream2 | `moondream2.gguf` | Processes image inputs (On-demand). |
| **Embedding** | BGE-M3-Small | (Library Load) | Semantic routing and RAG vector search. |

### 3.2. Architectural Diagram (Mermaid)

```mermaid
graph TD
    User[User Input] --> Router{Router Node<br/>(BGE-M3 Embedding)};
    
    Router -- "Complex Query" --> Reasoner[Reasoning Node<br/>(DeepSeek-R1)];
    Router -- "Simple Query" --> Drafter;
    
    Reasoner --> Drafter[Drafting Node<br/>(Nanbeige-4.1)];
    
    Drafter --> ReviewA[Review Node A<br/>(Gemma-3)];
    Drafter --> ReviewB[Review Node B<br/>(SmolLM2)];
    
    ReviewA --> Synthesis[Synthesis Node<br/>(Phi-4-mini)];
    ReviewB --> Synthesis;
    
    Synthesis --> Final[Final Output];
```

## 4. Data Flow & State Management

**State (`AgentState`):**
-   `query`: Original user question.
-   `is_complex`: Boolean flag determined by router.
-   `logic_map`: Output from DeepSeek-R1 (CoT).
-   `draft`: Initial response from Nanbeige.
-   `critique_gemma`: Feedback from Gemma-3.
-   `critique_smollm`: Feedback from SmolLM2.
-   `final_answer`: Final synthesized response from Phi-4.

**Workflow:**
1.  **Router:** Analyzes query complexity using embeddings.
2.  **Reasoner:** Creates a logical structure for the answer.
3.  **Drafter:** Writes a full draft based on the logic map.
4.  **Review (Parallel/Sequential):** Two reviewers critique the draft from different perspectives (Style vs. Facts). *Note: On limited hardware, these run sequentially but logically represent parallel critiques.*
5.  **Synthesis:** Combines the draft and critiques into a polished final answer.

## 5. Memory Management Strategy

To operate within 16GB RAM:
1.  **Single Active LLM:** Only one GGUF model is loaded into RAM at a time.
2.  **Aggressive Unloading:** `ModelManager` calls `del model` and `gc.collect()` immediately after a node completes its inference.
3.  **Resident Embeddings:** BGE-M3 remains in VRAM (approx 300MB) to avoid reloading latency for routing/RAG.
4.  **Swap Space:** OS Swap may be utilized but should be minimized to maintain performance.

## 6. Directory Structure

```
D:\llmassist\
├── models\             # GGUF model files
├── prompts\            # System prompt text files
├── model_manager.py    # Resource-aware model loader
├── mvp_agent_v2.py     # LangGraph workflow definition
├── sdd.md              # This design document
└── ROADMAP.md          # Implementation plan
```

## 7. Future Enhancements

-   **RAG Implementation:** FAISS index for document retrieval using BGE-M3.
-   **Vision Integration:** Integrate Moondream2 node for image inputs.
-   **Latency Optimization:** Explore `llama.cpp` server mode to reduce model load overhead if feasible within RAM limits.
