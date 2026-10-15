#include "ChatCLI.h"

#include <iostream>
#include <string>


ChatCLI::ChatCLI(LLMSession& session)
    : session_(session)
{
}


void ChatCLI::setDebugMode(bool enabled) noexcept
{
    debugMode_ = enabled;

    // Enable or disable diagnostics inside the session too.
    session_.setDebugMode(enabled);
}


bool ChatCLI::runChat()
{
    if(!session_.isInitialized() && !session_.initialize())
    {
        std::cerr << "Failed to initialize session.\n";
        return false;
    }

    if(debugMode_)
    {
        std::clog << "[ChatCLI] Agent: "
                  << session_.agentConfig().id << '\n';

        std::clog << "[ChatCLI] Chat started.\n";
    }

    std::string userMessage;

    while(true)
    {
        std::cout << "\nYou: ";

        if(!std::getline(std::cin, userMessage))
        {
            break;
        }

        if(userMessage == "exit")
        {
            break;
        }

        if(userMessage.empty())
        {
            continue;
        }

        if(debugMode_)
        {
            std::clog << "[ChatCLI] Processing user message.\n";
        }

        std::string response;

        if(!session_.chat(userMessage, response))
        {
            std::cerr << "Chat generation failed.\n";
            continue;
        }

        std::cout << '\n'
                  << session_.agentConfig().id << ": "
                  << response << '\n';

        if(debugMode_)
        {
            std::clog << "[ChatCLI] Response completed.\n";
        }
    }

    if(debugMode_)
    {
        std::clog << "[ChatCLI] Chat ended.\n";
    }

    return true;
}