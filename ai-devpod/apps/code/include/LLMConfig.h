#pragma once

#include "llama.h"

#include <cstdint>
#include <string>
#include <vector>

// ============================================================
// MODEL CONFIGURATION
// ============================================================
// Controls which model is loaded and how its weights are loaded.
// These settings generally remain fixed for the lifetime of the loaded model.

struct ModelConfig
{
    std::string id = "default";     // Identification
    std::string model_path;         // Model weights
    int32_t gpu_layers = -1;        //-1 means all layers, subject to available backend resources.
    int32_t main_gpu = 0;           // Hardware
    enum llama_split_mode split_mode = LLAMA_SPLIT_MODE_LAYER; // Multi-GPU distribution
    bool check_tensors = false;     // Validation
};


// ============================================================
// CONTEXT CONFIGURATION
// ============================================================
// Controls the execution context associated with a conversation.
// Different agents or sessions can use different context settings.

struct ContextConfig
{
    uint32_t context_size = 8192; // Maximum context length in tokens.
    uint32_t batch_size = 512; // Maximum logical batch size.
    uint32_t ubatch_size = 128; // Maximum physical batch size.
    uint32_t max_sequences = 1; // Maximum number of sequences supported by this context.
    int32_t threads = 8;    // CPU threading
    int32_t threads_batch = 8;  // CPU threading
    bool offload_kqv = true;    // Hardware/runtime behavior
    bool measure_performance = false;   // Performance measurements
};


// ============================================================
// SAMPLING CONFIGURATION
// ============================================================
// Controls token selection during text generation.
// These values are not used when loading the model.

struct SamplingConfig
{
    float temperature = 0.7f;   // Token selection
    int32_t top_k = 40;
    float top_p = 0.95f;
    float min_p = 0.05f;
    float repetition_penalty = 1.0f;    // Repetition control 1.0 means no repetition penalty.
    int32_t penalty_last_n = 64;
    float frequency_penalty = 0.0f; // Frequency/presence penalties
    float presence_penalty = 0.0f;
    int64_t seed = -1;  // Generation randomness -1 means the application should choose a random seed.
};


// ============================================================
// AGENT CONFIGURATION
// ============================================================
// Defines the behavior of a specific agent.
// An agent refers to a model by ID, so multiple agents can use the same underlying weights.

struct AgentConfig
{
    std::string id; // Agent identification
    std::string model_id = "default";   // ID of the model this agent uses.
    std::string system_prompt;  // Instructions that define the agent's role.
    ContextConfig context;  // Per-agent execution context.
    SamplingConfig sampling;    // Per-agent sampling behavior.
    uint32_t max_new_tokens = 512; // Maximum number of tokens generated in one response.
    std::vector<std::string> stop_sequences;    // Optional strings that terminate generation.
};