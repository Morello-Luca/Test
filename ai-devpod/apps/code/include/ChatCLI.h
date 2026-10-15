#pragma once

#include "LLMSession.h"

class ChatCLI
{
public:
    explicit ChatCLI(LLMSession& session);

    bool runChat();

    void setDebugMode(bool enabled) noexcept;

private:
    LLMSession& session_;

    bool debugMode_ = false;
};