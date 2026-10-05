# API Reference

Base URL: `http://localhost:8010`.

## Health

```bash
curl localhost:8010/health
```

```json
{ "status": "ok", "version": "0.1.0" }
```

## Summarize

```bash
curl -X POST localhost:8010/api/v1/summarize \
  -H 'content-type: application/json' \
  -d '{"note":"62-year-old with type 2 diabetes and hypertension presents with chest pain and shortness of breath. Home medications include metformin and lisinopril. Plan: continue current therapy; follow up in two weeks."}'
```

```json
{
  "summary": {
    "findings": ["chest pain", "shortness of breath"],
    "medications": ["metformin", "lisinopril"],
    "conditions": ["diabetes", "hypertension"],
    "summary": "62-year-old with type 2 diabetes and hypertension presents with chest pain…",
    "backend": "rule",
    "disclaimer": "Automated extraction from a clinical note. Demo output — not medical advice; verify before any clinical use."
  },
  "input_chars": 196
}
```

- `backend` is `"rule"` or `"llm"` — the same shape either way
- In LLM mode, a failed LLM call silently degrades to the rules (the
  `backend` field tells you it happened)

## Errors

| Status | Meaning |
|--------|---------|
| `400` | empty note |
| `502` | never — the fallback chain guarantees a response |
