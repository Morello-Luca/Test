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
        // Controls the execution context associated with a conversation. different agents or sessions can use different context settings.
struct ContextConfig {
    uint32_t context_size = 8192;      // Larger: longer conversations and more input, but uses more memory.
    uint32_t batch_size = 512;         // Larger: faster prompt processing, but uses more memory.
    uint32_t ubatch_size = 128;        // Larger: potentially faster processing, but increases peak memory usage.
    uint32_t max_sequences = 1;        // Larger: more concurrent conversations, but increases resource requirements.
    int32_t threads = 8;               // More threads: potentially faster generation on CPU, but with diminishing returns.
    int32_t threads_batch = 8;         // More threads: potentially faster prompt processing on CPU, but uses more CPU resources.
    bool offload_kqv = true;           // True: offloads KQV operations and KV cache to GPU when supported; uses more GPU memory.
    bool measure_performance = false;  // True: enables performance measurements for profiling inference.
};

// ============================================================
// SAMPLING CONFIGURATION
        // Controls token selection during text generation. these values are not used when loading the model.
struct SamplingConfig {
    float temperature = 0.7f;          // Higher: more creative and random; lower: more predictable and focused.
    int32_t top_k = 40;                // Higher: more token choices; lower: more restricted and predictable.
    float top_p = 0.95f;               // Higher: more token diversity; lower: restricts choices to more likely tokens.
    float min_p = 0.05f;               // Higher: filters more unlikely tokens; lower: allows more diversity.
    float repetition_penalty = 1.0f;   // Higher: discourages repeated tokens; 1.0: no penalty.
    int32_t penalty_last_n = 64;       // Higher: considers more previous tokens for repetition penalties.
    float frequency_penalty = 0.0f;    // Higher: more strongly discourages frequently repeated tokens.
    float presence_penalty = 0.0f;     // Higher: discourages reusing tokens that already appeared.
    int64_t seed = -1;                 // Fixed seed: reproducible randomness; -1: choose a random seed.
};


enum class ThinkingPolicy
{
    AlwaysOff,   // This agent never thinks
    AlwaysOn,    // This agent always thinks
    PerMessage   // Each message can override the default
};

// ============================================================
// AGENT CONFIGURATION
        // Defines the behavior of a specific agent. An agent refers to a model by ID, so multiple agents can use the same underlying weights.
struct AgentConfig
{
std::string id;                          // Identifies the agent.
std::string model_id = "default";        // Selects the model.
std::string system_prompt;               // Defines the agent's role, behavior, and instructions; more detailed prompts use more context.

ThinkingPolicy thinking = ThinkingPolicy::PerMessage;
bool default_thinking = false;

ContextConfig context;                   // Controls context capacity, processing speed, and CPU/GPU resource usage.
SamplingConfig sampling;                 // Controls response randomness, diversity, and repetition.
uint32_t max_new_tokens = 512;           // Higher: allows longer responses but increases generation time and resource usage.
std::vector<std::string> stop_sequences; // Strings that stop generation when encountered; useful for controlling response boundaries.
};