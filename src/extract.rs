//! Extraction: rule-based (default, deterministic) and LLM (Ollama,
//! schema-constrained) backends producing the same typed summary.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuredSummary {
    /// Patient-relevant findings (short phrases).
    pub findings: Vec<String>,
    /// Medications mentioned.
    pub medications: Vec<String>,
    /// Conditions/diagnoses mentioned.
    pub conditions: Vec<String>,
    /// One-paragraph summary of the note.
    pub summary: String,
    /// "rule" or "llm"
    pub backend: String,
    pub disclaimer: String,
}

pub const DISCLAIMER: &str = "Automated extraction from a clinical note. Demo \
output — not medical advice; verify before any clinical use.";

/// Known medication/condition keyword lexicon for the rule backend —
/// deliberately small and transparent for the scaffold; the LLM backend
/// generalizes.
const MEDICATIONS: [&str; 16] = [
    "aspirin",
    "metformin",
    "lisinopril",
    "insulin",
    "warfarin",
    "atorvastatin",
    "amlodipine",
    "metoprolol",
    "albuterol",
    "omeprazole",
    "gabapentin",
    "prednisone",
    "furosemide",
    "levothyroxine",
    "ibuprofen",
    "sertraline",
];
const CONDITIONS: [&str; 14] = [
    "diabetes",
    "hypertension",
    "asthma",
    "copd",
    "atrial fibrillation",
    "heart failure",
    "hyperlipidemia",
    "hypothyroidism",
    "anemia",
    "depression",
    "anxiety",
    "gout",
    "migraine",
    "pneumonia",
];
const FINDING_CUES: [&str; 8] = [
    "chest pain",
    "shortness of breath",
    "fatigue",
    "fever",
    "cough",
    "headache",
    "dizziness",
    "swelling",
];

/// Deterministic rule extractor: keyword scan + a lead-paragraph summary.
pub fn extract_rules(note: &str) -> StructuredSummary {
    let lower = note.to_lowercase();
    let findings = FINDING_CUES
        .iter()
        .filter(|cue| lower.contains(*cue))
        .map(|cue| cue.to_string())
        .collect();
    let medications = MEDICATIONS
        .iter()
        .filter(|med| lower.contains(*med))
        .map(|med| med.to_string())
        .collect();
    let conditions = CONDITIONS
        .iter()
        .filter(|condition| lower.contains(*condition))
        .map(|condition| condition.to_string())
        .collect();
    let summary = lead_summary(note);
    StructuredSummary {
        findings,
        medications,
        conditions,
        summary,
        backend: "rule".into(),
        disclaimer: DISCLAIMER.into(),
    }
}

fn lead_summary(note: &str) -> String {
    let first_paragraph = note
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default();
    let words: Vec<&str> = first_paragraph.split_whitespace().collect();
    let short: Vec<&str> = words.into_iter().take(40).collect();
    if short.is_empty() {
        "No summary (empty note).".to_string()
    } else {
        format!("{}…", short.join(" "))
    }
}

/// LLM extractor: Ollama with a JSON schema constraint. Requires a
/// running model; callers should offer the rule backend as fallback.
pub async fn extract_llm(
    http: &reqwest::Client,
    ollama_url: &str,
    model: &str,
    note: &str,
) -> Result<StructuredSummary, String> {
    #[derive(Debug, Deserialize)]
    struct ChatResponse {
        message: ChatMessage,
    }
    #[derive(Debug, Deserialize)]
    struct ChatMessage {
        content: String,
    }

    let schema = serde_json::json!({
        "type": "object",
        "properties": {
            "findings": {"type": "array", "items": {"type": "string"}},
            "medications": {"type": "array", "items": {"type": "string"}},
            "conditions": {"type": "array", "items": {"type": "string"}},
            "summary": {"type": "string"}
        },
        "required": ["findings", "medications", "conditions", "summary"]
    });
    let response: ChatResponse = http
        .post(format!("{ollama_url}/api/chat"))
        .json(&serde_json::json!({
            "model": model,
            "messages": [
                {"role": "system", "content": "Extract a structured summary from the \
                 clinical note. Return findings, medications, and conditions as short \
                 phrases. You are not providing medical advice."},
                {"role": "user", "content": note}
            ],
            "format": schema,
            "stream": false,
            "options": {"temperature": 0.1}
        }))
        .send()
        .await
        .map_err(|e| format!("ollama unreachable: {e}"))?
        .error_for_status()
        .map_err(|e| format!("ollama status: {e}"))?
        .json()
        .await
        .map_err(|e| format!("ollama body: {e}"))?;

    let mut summary: StructuredSummary = serde_json::from_str(&response.message.content)
        .map_err(|e| format!("model output did not match schema: {e}"))?;
    summary.backend = "llm".into();
    summary.disclaimer = DISCLAIMER.into();
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTE: &str = "\
62-year-old with type 2 diabetes and hypertension presents with chest pain \
and shortness of breath. Home medications include metformin and lisinopril.\
Plan: continue current therapy; follow up in two weeks.";

    #[test]
    fn rule_extractor_finds_structured_fields() {
        let summary = extract_rules(NOTE);
        assert!(summary.medications.contains(&"metformin".to_string()));
        assert!(summary.medications.contains(&"lisinopril".to_string()));
        assert!(summary.conditions.contains(&"diabetes".to_string()));
        assert!(summary.conditions.contains(&"hypertension".to_string()));
        assert!(summary.findings.contains(&"chest pain".to_string()));
        assert!(summary.summary.contains("62-year-old"));
        assert_eq!(summary.backend, "rule");
        assert!(summary.disclaimer.contains("not medical advice"));
    }

    #[test]
    fn empty_note_is_graceful() {
        let summary = extract_rules("   \n  \n");
        assert!(summary.medications.is_empty());
        assert!(summary.findings.is_empty());
    }

    #[test]
    fn no_false_positives_on_unrelated_text() {
        let summary = extract_rules("Patient ate lunch and went for a walk.");
        assert!(summary.medications.is_empty());
        assert!(summary.conditions.is_empty());
        assert!(summary.findings.is_empty());
    }
}
