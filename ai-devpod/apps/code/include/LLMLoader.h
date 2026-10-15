#pragma once

#include "llama.h"
#include "LLMConfig.h"

#include <memory>
#include <vector>

class LLMLoader
{
public:
    explicit LLMLoader(const ModelConfig& config);
    ~LLMLoader();

    LLMLoader(const LLMLoader&) = delete;
    LLMLoader& operator=(const LLMLoader&) = delete;

    bool load();
    void unload();

    bool isLoaded() const noexcept;

    llama_model* model() const noexcept;

    const ModelConfig& config() const noexcept;

    static void setDebugMode(bool enabled) noexcept;

    // Find and load the model required by an agent.
    static std::unique_ptr<LLMLoader> loadForAgent(const AgentConfig& agent,const std::vector<ModelConfig>& availableModels);

private:
    ModelConfig config_;
    llama_model* model_ = nullptr;
    llama_context* context_ = nullptr;
};