# Data Flow Architecture

Data flow in RustAI is strictly unidirectional across async event channels.

```
[ User Input / GUI Event ]
          │
          ▼
   (rust-ai-app) ──[ AppEvent ]──> (rust-ai-core Task Loop)
                                          │
                                          ├──> [ ModelProvider::stream() ]
                                          │             │
   (rust-ai-app) <──[ StreamChunk ]───────┴─────────────┘
```

## Key Characteristics
* **Streaming Responses**: Token streams from AI model backends are pushed to UI channels as async `StreamChunk` events, keeping the interface responsive.
* **Non-Blocking UI**: Long-running inference and tool calls operate on Tokio background worker threads.
