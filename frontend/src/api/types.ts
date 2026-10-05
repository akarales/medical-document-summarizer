export interface StructuredSummary {
  findings: string[];
  medications: string[];
  conditions: string[];
  summary: string;
  backend: string;
  disclaimer: string;
}

export interface SummarizeResponse {
  summary: StructuredSummary;
  input_chars: number;
}
