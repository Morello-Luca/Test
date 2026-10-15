#pragma once

#include "llama.h"
#include "LLMConfig.h"
#include "LLMLoader.h"

#include <string>
#include <vector>

class LLMSession
{
public:
    LLMSession(
        LLMLoader& model,
        const AgentConfig& agent
    );

    ~LLMSession();

    LLMSession(const LLMSession&) = delete;
    LLMSession& operator=(const LLMSession&) = delete;

    bool initialize();
    void close();

    bool isInitialized() const noexcept;

    llama_context* context() const noexcept;

    const ContextConfig& config() const noexcept;

    const AgentConfig& agentConfig() const noexcept;

    // Send a message and generate a response.
    // Conversation history is preserved between calls.
    bool chat(
        const std::string& userMessage,
        std::string& response
    );

    // Start a new conversation without reloading the model.
    void clearHistory();

    void setDebugMode(bool enabled) noexcept;
private:

    bool debugMode_ = false;

    struct Message
    {
        std::string role;
        std::string content;
    };

    bool formatHistory(
        bool addAssistantPrompt,
        std::string& formatted
    ) const;

    bool evaluateTokens(
        const llama_token* tokens,
        int32_t count,
        bool outputLogits
    );

    bool generateText(
        const std::string& prompt,
        std::string& response
    );

    // The model must outlive this session.
    LLMLoader& model_;

    // This session's agent configuration.
    AgentConfig agent_;

    llama_context* context_ = nullptr;

    // Conversation history belonging to this session.
    std::vector<Message> history_;

    // Length of the previously formatted conversation.
    // Used to avoid evaluating the entire history again.
    int32_t previousFormattedLength_ = 0;
};