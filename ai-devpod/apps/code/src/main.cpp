#include "agents.h"
#include "LLMLoader.h"
#include "LLMSession.h"
#include "ChatCLI.h"

int main()
{
    constexpr bool debugMode = false;
    LLMLoader::setDebugMode(debugMode);

    // Select the agent.
    AgentConfig agent = makeCodingAgent();

    // Load its model.
    auto loader = LLMLoader::loadForAgent(agent, models);

    if(!loader)
    {
        return 1;
    }

    // Create the session.
    LLMSession session(*loader, agent);

    // Create and run the CLI.
    ChatCLI cli(session);
    cli.setDebugMode(debugMode);

    return cli.runChat() ? 0 : 1;
}



    /*
AgentConfig coder1 = makeCodingAgent();
coder1.id = "coding_1";

AgentConfig coder2 = makeCodingAgent();
coder2.id = "coding_2";
    */

