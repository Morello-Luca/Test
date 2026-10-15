#include "LLMLoader.h"

#include <utility>
#include <algorithm>
#include <iostream>
#include <memory>
#include <filesystem>
#include <fstream>
#include <cstdio>



namespace
{
    bool g_llamaDebugMode = false;

    void llamaLogCallback(
        enum ggml_log_level level,
        const char* text,
        void* userData)
    {
        const bool debug = *static_cast<const bool*>(userData);

        // Debug mode: display everything.
        if(debug)
        {
            std::fputs(text, stderr);
            std::fflush(stderr);
            return;
        }

        // Normal mode: display warnings and errors only.
        if(level == GGML_LOG_LEVEL_ERROR)
        {
            std::fputs(text, stderr);
            std::fflush(stderr);
        }
    }
}


void LLMLoader::setDebugMode(bool enabled) noexcept
{
    g_llamaDebugMode = enabled;
    llama_log_set( llamaLogCallback,&g_llamaDebugMode );
}

LLMLoader::LLMLoader(const ModelConfig& config): config_(config)
{
}


LLMLoader::~LLMLoader()
{
    unload();
}

void LLMLoader::unload()
{
    if (model_ != nullptr)
    {
        llama_model_free(model_);
        model_ = nullptr;
    }
}

bool LLMLoader::isLoaded() const noexcept
{
    return model_ != nullptr;
}

llama_model* LLMLoader::model() const noexcept
{
    return model_;
}

const ModelConfig& LLMLoader::config() const noexcept
{
    return config_;
}

std::unique_ptr<LLMLoader> LLMLoader::loadForAgent( const AgentConfig& agent, const std::vector<ModelConfig>& availableModels)
{
    // Find the model associated with this agent.
    auto it = std::find_if( availableModels.begin(), availableModels.end(), [&](const ModelConfig& model)
        {
            return model.id == agent.model_id;
        }
    );

    if(it == availableModels.end())
    {
        std::cerr << "Model not found: " << agent.model_id << '\n';
        return nullptr;
    }

    // Create the loader.
    auto loader = std::make_unique<LLMLoader>(*it);

    // Load the model weights.
    if(!loader->load())
    {
        std::cerr << "Failed to load model: " << agent.model_id << '\n';
        return nullptr;
    }

    return loader;
}

bool LLMLoader::load()
{
    // Already loaded.
    if(model_ != nullptr)
    {
        return true;
    }

    // Check the configured path.
    if(config_.model_path.empty())
    {
        std::cerr << "Model path is empty.\n";
        return false;
    }

    // Check that the file exists.
    std::error_code ec;

    if(!std::filesystem::is_regular_file(config_.model_path, ec))
    {
        std::cerr << "Model file does not exist or is not " "a regular file: " << config_.model_path;

        if(ec)
        {
            std::cerr << " (" << ec.message() << ")";
        }

        std::cerr << '\n';
        return false;
    }

    // Check that this process can open the file for reading.
    std::ifstream file( config_.model_path, std::ios::binary);

    if(!file)
    {
        std::cerr << "Cannot read model file: " << config_.model_path << '\n';
        return false;
    }

    file.close();

    // Configure llama.cpp.
    llama_model_params params = llama_model_default_params();

    params.n_gpu_layers = config_.gpu_layers;
    params.main_gpu = config_.main_gpu;
    params.split_mode = config_.split_mode;
    params.check_tensors = config_.check_tensors;

    // Load the model.
    model_ = llama_model_load_from_file(config_.model_path.c_str(),params
    );

    if(model_ == nullptr)
    {
        std::cerr << "llama.cpp failed to load model: " << config_.model_path << '\n';
        return false;
    }

    return true;
}