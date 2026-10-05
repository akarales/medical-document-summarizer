# Development Guide

Machine-facing commands live in [AGENTS.md](../AGENTS.md).

## Prerequisites

Rust 1.96, cargo · pnpm 11 / Node 24 · optional Ollama for LLM mode.

## Daily loop

```bash
cargo run                    # :8010 — rule extractor
APP_USE_LLM=true cargo run   # LLM extractor with rule fallback
cd frontend && pnpm dev       # :5173 → /api proxied
cargo test -q                # 6 tests — extractor + contract
cargo clippy --all-targets -- -D warnings
```

## Extending the lexicon (safely)

The rule backend's keyword tables are policy — extend them WITH tests:

1. Add the term to `MEDICATIONS`, `CONDITIONS`, or `FINDING_CUES` in
   `src/extract.rs`
2. Add the term to the extraction unit test (and keep the
   no-false-positive test green)
3. Watch for substring hazards — "insulin" is fine; a term that is a
   substring of an unrelated word is a bug

## Testing notes

- Extractor tests pin semantics: known terms extracted, empty notes
  graceful, unrelated text yields nothing (the false-positive guard)
- Contract tests cover validation and the typed response shape

## Gotchas learned here

- Port 8010
- Ollama schema mismatches are errors, not guesses — the fallback chain
  exists precisely because models drift

## Conventions

Conventional commits; hygiene hook strips AI attribution. Both
backends must produce the same typed summary — never add fields to one
without the other.
