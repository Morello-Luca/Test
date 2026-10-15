#include "LLMSession.h"

#include <algorithm>
#include <iostream>
#include <memory>
#include <vector>


LLMSession::LLMSession( LLMLoader& model, const AgentConfig& agent )
    : model_(model),
      agent_(agent)
{
}


LLMSession::~LLMSession()
{
    close();
}


bool LLMSession::initialize()
{
    if(context_ != nullptr)
    {
        return true;
    }

    if(!model_.isLoaded())
    {
        std::cerr << "Cannot initialize session: model is not loaded.\n";
        return false;
    }

    llama_context_params params = llama_context_default_params();
    // CONTEXT CONFIGURATION
        params.n_ctx = agent_.context.context_size;
        params.n_batch = agent_.context.batch_size;
        params.n_ubatch = agent_.context.ubatch_size;
        params.n_seq_max = agent_.context.max_sequences;
        params.n_threads = agent_.context.threads;
        params.n_threads_batch = agent_.context.threads_batch;
        params.offload_kqv = agent_.context.offload_kqv;
        params.no_perf = !agent_.context.measure_performance;

    context_ = llama_init_from_model( model_.model(), params);

    if(context_ == nullptr)
    {
        std::cerr << "Failed to initialize session context.\n";
        return false;
    }

    return true;
}


void LLMSession::close()
{
    if(context_ != nullptr)
    {
        llama_free(context_);
        context_ = nullptr;
    }

    history_.clear();
    previousFormattedLength_ = 0;
}


bool LLMSession::isInitialized() const noexcept
{
    return context_ != nullptr;
}


llama_context* LLMSession::context() const noexcept
{
    return context_;
}


const ContextConfig& LLMSession::config() const noexcept
{
    return agent_.context;
}


const AgentConfig& LLMSession::agentConfig() const noexcept
{
    return agent_;
}

bool LLMSession::formatHistory( bool addAssistantPrompt, std::string& formatted ) const
{
    const llama_model* model = model_.model();
    const char* chatTemplate = llama_model_chat_template(model, nullptr);

    std::vector<llama_chat_message> messages;
    messages.reserve(history_.size());

    for(const auto& message : history_)
    {
        messages.push_back({ message.role.c_str(), message.content.c_str() });
    }

    // First determine the required buffer size.
    const int32_t required = llama_chat_apply_template(
        chatTemplate,
        messages.data(),
        messages.size(),
        addAssistantPrompt,
        nullptr,
        0
    );

    if(required < 0)
    {
        std::cerr << "Failed to apply model chat template.\n";
        return false;
    }

    // Allocate enough space, including the null terminator.
    std::vector<char> buffer( static_cast<std::size_t>(required) + 1, '\0' );

    const int32_t written = llama_chat_apply_template(
        chatTemplate,
        messages.data(),
        messages.size(),
        addAssistantPrompt,
        buffer.data(),
        static_cast<int32_t>(buffer.size())
    );

    if(written < 0 || written > required)
    {
        std::cerr << "Failed to format conversation.\n";
        return false;
    }

    formatted.assign( buffer.data(), static_cast<std::size_t>(written) );
    return true;
}


void LLMSession::setDebugMode(bool enabled) noexcept
{
    debugMode_ = enabled;
}


bool LLMSession::evaluateTokens(
    const llama_token* tokens,
    int32_t count,
    bool outputLogits
)
{
    if(context_ == nullptr || tokens == nullptr || count <= 0)
    {
        return false;
    }

    const llama_pos startPosition = llama_memory_seq_pos_max( llama_get_memory(context_), 0) + 1;

    const int64_t endPosition = static_cast<int64_t>(startPosition) + count;

    if(endPosition > static_cast<int64_t>(llama_n_ctx(context_)))
    {
        std::cerr << "Context window is full. " << "Clear the conversation or start a new session.\n";
        return false;
    }

    const int32_t batchSize = static_cast<int32_t>( std::max<uint32_t>(1, llama_n_batch(context_)) );

    int32_t offset = 0;

    while(offset < count)
    {
        const int32_t currentCount = std::min( batchSize, count - offset );

    llama_batch batch = llama_batch_init( currentCount, 0, 1 );

if(batch.token == nullptr ||
   batch.pos == nullptr ||
   batch.n_seq_id == nullptr ||
   batch.seq_id == nullptr ||
   batch.logits == nullptr)
{
    llama_batch_free(batch);
    std::cerr << "Failed to allocate token batch.\n";
    return false;
}

// IMPORTANT: set the number of active tokens.
batch.n_tokens = currentCount;

for(int32_t i = 0; i < currentCount; ++i)
{
    const int32_t tokenIndex = offset + i;

    batch.token[i] = tokens[tokenIndex];
    batch.pos[i] = startPosition + tokenIndex;

    batch.n_seq_id[i] = 1;
    batch.seq_id[i][0] = 0;

    batch.logits[i] =
        (outputLogits && tokenIndex == count - 1) ? 1 : 0;
}

const int result = llama_decode(context_, batch);

llama_batch_free(batch);

if(result != 0)
{
    std::cerr << "Failed to evaluate tokens. Code: "
              << result << '\n';
    return false;
}

offset += currentCount;
    }

    return true;
}


bool LLMSession::generateText(const std::string& prompt, std::string& response)
{
    if(context_ == nullptr || !model_.isLoaded())
    {
        return false;
    }

    if(agent_.max_new_tokens == 0)
    {
        std::cerr << "max_new_tokens must be greater than zero.\n";
        return false;
    }

    response.clear();

    const llama_model* model = model_.model();
    const llama_vocab* vocab = llama_model_get_vocab(model);

    // Determine whether this context has processed any tokens.
    const bool isFirst =
        llama_memory_seq_pos_max(
            llama_get_memory(context_),
            0
        ) == -1;

    // Query the required number of tokens.
    const int32_t required = -llama_tokenize(
        vocab,
        prompt.c_str(),
        static_cast<int32_t>(prompt.size()),
        nullptr,
        0,
        isFirst,
        true
    );

    if(required <= 0)
    {
        std::cerr << "Failed to tokenize prompt.\n";
        return false;
    }

    std::vector<llama_token> promptTokens(
        static_cast<std::size_t>(required)
    );

    const int32_t tokenCount = llama_tokenize(
        vocab,
        prompt.c_str(),
        static_cast<int32_t>(prompt.size()),
        promptTokens.data(),
        static_cast<int32_t>(promptTokens.size()),
        isFirst,
        true
    );

    if(tokenCount < 0)
    {
        std::cerr << "Prompt tokenization failed.\n";
        return false;
    }

    promptTokens.resize(static_cast<std::size_t>(tokenCount));

    // Create the sampler chain for this response.
    using SamplerPtr =
        std::unique_ptr<llama_sampler, decltype(&llama_sampler_free)>;

    SamplerPtr sampler(
        llama_sampler_chain_init(
            llama_sampler_chain_default_params()
        ),
        &llama_sampler_free
    );

    if(!sampler)
    {
        std::cerr << "Failed to initialize sampler.\n";
        return false;
    }

    auto addSampler = [&](llama_sampler* item)
    {
        if(item != nullptr)
        {
            llama_sampler_chain_add(sampler.get(), item);
        }
    };

    // Repetition and frequency penalties, if enabled.
    if(agent_.sampling.repetition_penalty != 1.0f ||
       agent_.sampling.frequency_penalty != 0.0f ||
       agent_.sampling.presence_penalty != 0.0f)
    {
        addSampler(
            llama_sampler_init_penalties(
                llama_vocab_n_tokens(vocab),
                agent_.sampling.penalty_last_n,
                agent_.sampling.repetition_penalty,
                agent_.sampling.frequency_penalty,
                agent_.sampling.presence_penalty
            )
        );
    }

    if(agent_.sampling.temperature <= 0.0f)
    {
        // Temperature zero: greedy decoding.
        addSampler(llama_sampler_init_greedy());
    }
    else
    {
        if(agent_.sampling.top_k > 0)
        {
            addSampler(
                llama_sampler_init_top_k(
                    agent_.sampling.top_k
                )
            );
        }

        if(agent_.sampling.top_p > 0.0f &&
           agent_.sampling.top_p < 1.0f)
        {
            addSampler(
                llama_sampler_init_top_p(
                    agent_.sampling.top_p,
                    1
                )
            );
        }

        if(agent_.sampling.min_p > 0.0f &&
           agent_.sampling.min_p < 1.0f)
        {
            addSampler(
                llama_sampler_init_min_p(
                    agent_.sampling.min_p,
                    1
                )
            );
        }

        addSampler(
            llama_sampler_init_temp(
                agent_.sampling.temperature
            )
        );

        const uint32_t seed =
            agent_.sampling.seed < 0
                ? LLAMA_DEFAULT_SEED
                : static_cast<uint32_t>(
                    agent_.sampling.seed
                  );

        addSampler(llama_sampler_init_dist(seed));
    }

    // The first input is the prompt. Subsequent inputs are
    // the tokens generated by the model.
    const llama_token* currentTokens = promptTokens.data();
    int32_t currentCount = tokenCount;

    llama_token pendingToken = LLAMA_TOKEN_NULL;
    bool pendingTokenNeedsEvaluation = false;

    for(uint32_t generated = 0;
        generated < agent_.max_new_tokens;
        ++generated)
    {
        if(!evaluateTokens(
            currentTokens,
            currentCount,
            true
        ))
        {
            return false;
        }

        // The current input has now been evaluated.
        pendingTokenNeedsEvaluation = false;

        const llama_token token = llama_sampler_sample(
            sampler.get(),
            context_,
            -1
        );

        // Stop when the model emits an end-of-generation token.
        if(llama_vocab_is_eog(vocab, token))
        {
            break;
        }

        // Convert the generated token into text.
        std::string piece(256, '\0');

        int32_t pieceSize = llama_token_to_piece(
            vocab,
            token,
            piece.data(),
            static_cast<int32_t>(piece.size()),
            0,
            true
        );

        // Retry with a larger buffer if necessary.
        if(pieceSize < 0)
        {
            piece.resize(
                static_cast<std::size_t>(-pieceSize)
            );

            pieceSize = llama_token_to_piece(
                vocab,
                token,
                piece.data(),
                static_cast<int32_t>(piece.size()),
                0,
                true
            );
        }

        if(pieceSize < 0)
        {
            std::cerr << "Failed to convert token to text.\n";
            return false;
        }

        piece.resize(static_cast<std::size_t>(pieceSize));
        response += piece;

        // Remember this token so it can be evaluated during
        // the next iteration, or flushed after the final token.
        pendingToken = token;
        pendingTokenNeedsEvaluation = true;

        currentTokens = &pendingToken;
        currentCount = 1;

        // Respect configured textual stop sequences.
        bool stopRequested = false;

        for(const auto& stop : agent_.stop_sequences)
        {
            if(stop.empty())
            {
                continue;
            }

            const std::size_t stopPosition = response.find(stop);

            if(stopPosition != std::string::npos)
            {
                response.resize(stopPosition);
                stopRequested = true;
                break;
            }
        }

        if(stopRequested)
        {
            break;
        }
    }

    // Make sure the last ordinary token is included in the
    // context, even when max_new_tokens or a stop sequence
    // terminates generation.
    if(pendingTokenNeedsEvaluation)
    {
        if(!evaluateTokens(
            &pendingToken,
            1,
            false
        ))
        {
            return false;
        }
    }

    return true;
}

bool LLMSession::chat(
    const std::string& userMessage,
    std::string& response
)
{
    if(!isInitialized())
    {
        std::cerr << "Session is not initialized.\n";
        return false;
    }

    // Add the system prompt only at the beginning
    // of a conversation.
    if(history_.empty() && !agent_.system_prompt.empty())
    {
        history_.push_back({
            "system",
            agent_.system_prompt
        });
    }

    history_.push_back({
        "user",
        userMessage
    });

    std::string formatted;

    if(!formatHistory(true, formatted))
    {
        history_.pop_back();
        return false;
    }

    if(previousFormattedLength_ < 0 ||
       static_cast<std::size_t>(previousFormattedLength_) >
           formatted.size())
    {
        std::cerr << "Invalid conversation history position.\n";
        clearHistory();
        return false;
    }

    // Only process the newly added part of the conversation.
    const std::string prompt =
        formatted.substr(
            static_cast<std::size_t>(previousFormattedLength_)
        );

    if(!generateText(prompt, response))
    {
        // Generation may have partially changed the context.
        // Reset both context and history to keep them consistent.
        clearHistory();
        return false;
    }

    history_.push_back({
        "assistant",
        response
    });

    // Save the formatted conversation length for the next turn.
    std::string completeHistory;

    if(!formatHistory(false, completeHistory))
    {
        clearHistory();
        return false;
    }

    previousFormattedLength_ =
        static_cast<int32_t>(completeHistory.size());

    return true;
}


void LLMSession::clearHistory()
{
    history_.clear();
    previousFormattedLength_ = 0;

    if(context_ != nullptr)
    {
        llama_memory_clear(
            llama_get_memory(context_),
            true
        );
    }
}