# Notes on ChatGPT Memory Design (Article Summary)

Source article: <https://manthanguptaa.in/posts/chatgpt_memory/>

This article proposes that ChatGPT memory is implemented as a layered context pipeline, not as full-history retrieval with vector search on every turn.

## Core Idea

The model likely combines four practical memory layers:

1. Session metadata (ephemeral): device, timezone, account/activity signals, and environment details.
2. User memory (long-term facts): explicit or inferred stable facts such as preferences and profile details.
3. Recent conversation summaries: lightweight snippets from recent chats for cross-session continuity.
4. Current session window: normal sliding window of full messages in the active chat.

Together with system/developer instructions and the latest user message, these layers build each prompt context.

## Why This Design Matters

- It is cheaper and faster than doing retrieval over full historical transcripts.
- It preserves personalization through explicit long-term facts.
- It keeps continuity with compact recent summaries.
- It still supports coherent reasoning through the current-session sliding window.

The trade-off is intentional: lower detail from old conversations in exchange for better latency, token efficiency, and operational simplicity.

## Important Caveat

The post is a reverse-engineering analysis based on observed behavior, not official implementation documentation. It is useful as an architectural hypothesis, not a guaranteed internal spec.
