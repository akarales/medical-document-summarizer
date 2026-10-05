# AGENTS.md

## Commands

```bash
cargo run                  # :8010 (rule extractor)
APP_USE_LLM=true cargo run # LLM extractor (needs Ollama)
cargo test -q              # 6 tests (3 extractor + 3 contract)
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cd frontend && pnpm install && pnpm dev && pnpm build
```

## Environment

`APP_PORT` (8010) · `APP_USE_LLM` (false) · `APP_OLLAMA_URL` ·
`APP_OLLAMA_MODEL` (qwen3:14b)

## Conventions

- Conventional commits; hygiene hook strips AI attribution
- Lexicon changes (medications/conditions/findings cues) live in
  `src/extract.rs` and are covered by tests — extend the table AND the
  tests together
- Both backends must produce the same typed summary; backend differences
  surface only via the `backend` field
- Deps ≥7 days old (BEST_PRACTICES/INDEX.md)
