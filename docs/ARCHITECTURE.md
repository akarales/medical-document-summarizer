# Architecture

## Modules

```
src/
├── extract.rs    # both backends: extract_rules + extract_llm
├── routes.rs     # summarize endpoint with the fallback chain
└── state.rs      # config (backend choice) + HTTP client
```

## The two backends (src/extract.rs)

Both produce the identical typed `StructuredSummary` — findings,
medications, conditions, summary, backend tag, disclaimer. The caller
cannot tell them apart except by `backend`.

**Rule extractor** (default):

- Keyword lexicons in one auditable place: 16 medications, 14
  conditions, 8 finding cues (chest pain, shortness of breath, fatigue,
  fever, …)
- Substring scan on the lowercased note + a lead-paragraph summary
  (first non-empty line, 40 words)
- Deterministic — the same note always yields the same summary
- Tests pin the no-false-positive property: "Patient ate lunch and went
  for a walk" extracts nothing

**LLM extractor** (`APP_USE_LLM=true`):

- Ollama `/api/chat` with a **JSON schema** in `format` — the model must
  return `{findings, medications, conditions, summary}` — at temperature
  0.1, system-prompted as extraction-only ("you are not providing
  medical advice")
- Output is parsed into the same typed struct; a schema mismatch is an
  error, not a guess

## The fallback chain (src/routes.rs)

`APP_USE_LLM=true`: try the LLM → on any failure (unreachable, bad
status, schema mismatch) log a warning and run the rules. The pipeline
never fails because a GPU is missing — the worst case is a shallower
summary, tagged `backend: "rule"`.

## Design decisions

| Decision | Why |
|----------|-----|
| Rules as the default | Deterministic demos, CI without a GPU, and a visible baseline to compare the LLM against |
| Same typed result from both | Callers and tests stay backend-agnostic; `backend` is observability, not a contract change |
| Lexicon in one file | Medication/condition tables are policy — extend them with tests, in the same commit |
| No silent metric pass-through | (Sibling principle to the wearable app) extraction fails loudly, not creatively |
