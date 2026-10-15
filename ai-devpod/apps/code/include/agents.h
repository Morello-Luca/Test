#pragma once

#include "LLMConfig.h"
#include <vector>
#include <string>
#include <unordered_map>


// ============================================================
// MODEL CONFIGURATIONS
// ============================================================
// curl -L -O https://huggingface.co/enacimie/Qwen3-0.6B-Q4_K_M-GGUF/resolve/main/qwen3-0.6b-q4_k_m.gguf
inline std::vector<ModelConfig> models = {
    {
        .id = "general",
        .model_path = "/workspace/models/qwen2.5-0.5b-instruct-q8_0.gguf",
        .gpu_layers = -1
    },
    {
        .id = "coder",
        .model_path = "/workspace/models/qwen2.5-0.5b-instruct-q8_0.gguf",
        .gpu_layers = -1
    }
};


// ============================================================
// AGENT CONFIGURATIONS
// ============================================================

inline AgentConfig makeResearchAgent()
{
    AgentConfig config;

    config.id = "research";
    config.model_id = "general";

    config.system_prompt =
        "You are a research assistant. "
        "Analyze technical questions carefully.";

    config.context.context_size = 8192;
    config.sampling.temperature = 0.3f;
    config.max_new_tokens = 2048;

    return config;
}


inline AgentConfig makeCodingAgent()
{
    AgentConfig config;

    config.id = "coding";
    config.model_id = "coder";

    config.system_prompt =
        "You are a C++ coding assistant. "
        "Write correct, maintainable code.";

    config.context.context_size = 8192;
    config.sampling.temperature = 0.2f;
    config.max_new_tokens = 2048;

    return config;
}