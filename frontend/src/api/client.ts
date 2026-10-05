import type { SummarizeResponse } from './types';

export async function postSummarize(note: string): Promise<SummarizeResponse> {
  const res = await fetch('/api/v1/summarize', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ note }),
  });
  if (!res.ok) {
    const body = await res.text();
    throw new Error(body || res.statusText);
  }
  return (await res.json()) as SummarizeResponse;
}
