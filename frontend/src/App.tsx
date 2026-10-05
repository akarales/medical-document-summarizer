import { useCallback, useState } from 'react';

import { postSummarize } from '@/api/client';
import type { SummarizeResponse } from '@/api/types';

const SAMPLE_NOTE = `62-year-old with type 2 diabetes and hypertension presents with chest pain and shortness of breath. Home medications include metformin and lisinopril. Reports fatigue for two weeks. Plan: continue current therapy; follow up in two weeks.`;

function Section({ title, items }: { title: string; items: string[] }) {
  return (
    <div className="rounded-lg border border-line bg-panel p-3">
      <h3 className="mb-1 text-xs font-semibold uppercase text-muted">{title}</h3>
      {items.length === 0 ? (
        <p className="text-xs text-muted">None detected.</p>
      ) : (
        <ul className="flex flex-wrap gap-1">
          {items.map((item) => (
            <li key={item} className="rounded bg-surface px-2 py-0.5 text-xs">
              {item}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}

export default function App() {
  const [note, setNote] = useState('');
  const [result, setResult] = useState<SummarizeResponse | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const run = useCallback(async (text: string) => {
    if (!text.trim()) return;
    setBusy(true);
    setError(null);
    setResult(null);
    try {
      setResult(await postSummarize(text));
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, []);

  return (
    <div className="flex h-full flex-col">
      <header className="border-b border-line bg-panel px-4 py-3">
        <h1 className="text-base font-semibold">Medical Document Summarizer</h1>
        <p className="text-xs text-muted">
          Clinical note → structured summary (findings, medications,
          conditions) — deterministic rule extractor by default, LLM when
          configured. Demo, not medical advice.
        </p>
      </header>

      <main className="mx-auto flex w-full max-w-4xl flex-1 flex-col gap-4 overflow-y-auto p-4">
        <section className="rounded-lg border border-line bg-panel p-4">
          <label className="mb-2 block text-sm font-semibold">Clinical note</label>
          <textarea
            value={note}
            onChange={(e) => setNote(e.target.value)}
            rows={8}
            placeholder="Paste a clinical note (synthetic, non-PHI)…"
            className="w-full rounded border border-line p-2 text-sm"
          />
          <div className="mt-2 flex flex-wrap gap-2">
            <button
              type="button"
              onClick={() => void run(note)}
              disabled={busy || !note.trim()}
              className="rounded bg-primary px-4 py-1.5 text-sm font-medium text-white disabled:opacity-50"
            >
              {busy ? 'Extracting…' : 'Summarize'}
            </button>
            <button
              type="button"
              onClick={() => {
                setNote(SAMPLE_NOTE);
                void run(SAMPLE_NOTE);
              }}
              className="rounded border border-line px-3 py-1.5 text-xs text-muted hover:bg-surface"
            >
              Use sample note
            </button>
          </div>
        </section>

        {error && (
          <p className="rounded border border-red-300 bg-red-50 p-3 text-sm text-red-800">
            {error}
          </p>
        )}

        {result && (
          <section className="flex flex-col gap-3">
            <div className="rounded-lg border border-line bg-panel p-3">
              <h3 className="mb-1 text-xs font-semibold uppercase text-muted">
                Summary (backend: {result.summary.backend})
              </h3>
              <p className="text-sm">{result.summary.summary}</p>
            </div>
            <div className="grid grid-cols-1 gap-3 md:grid-cols-3">
              <Section title="Findings" items={result.summary.findings} />
              <Section title="Medications" items={result.summary.medications} />
              <Section title="Conditions" items={result.summary.conditions} />
            </div>
            <p className="text-[10px] italic text-muted">
              {result.summary.disclaimer} ({result.input_chars} chars in)
            </p>
          </section>
        )}
      </main>
    </div>
  );
}
