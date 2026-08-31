//! DealShield Core
//!
//! Air-Gapped Investment Pitch & IP Redaction SDK.
//! License: AGPLv3

use regex::Regex;

pub struct RedactionResult {
    pub sanitized_text: String,
    pub redaction_count: usize,
}

pub struct DealShieldCore {
    currency_regex: Regex,
    email_regex: Regex,
}

impl Default for DealShieldCore {
    fn default() -> Self {
        Self::new()
    }
}

impl DealShieldCore {
    pub fn new() -> Self {
        Self {
            currency_regex: Regex::new(r"[$€£]\d+(?:,\d{3})*(?:\.\d{2})?").unwrap(),
            email_regex: Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}").unwrap(),
        }
    }

    /// Basic regex pattern matching (Community Edition)
    /// Masks emails and generic currency figures.
    pub fn sanitize_text(&self, input: &str) -> RedactionResult {
        let mut count = 0;
        let mut result = input.to_string();

        let before_len = result.len();
        result = self
            .email_regex
            .replace_all(&result, "[EMAIL_REDACTED]")
            .to_string();
        if result.len() != before_len {
            count += 1;
        } // simplified count

        let before_len = result.len();
        result = self
            .currency_regex
            .replace_all(&result, "[CURRENCY_REDACTED]")
            .to_string();
        if result.len() != before_len {
            count += 1;
        }

        RedactionResult {
            sanitized_text: result,
            redaction_count: count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_redaction() {
        let shield = DealShieldCore::new();
        let result = shield.sanitize_text("Contact ceo@startup.com for details.");
        assert_eq!(
            result.sanitized_text,
            "Contact [EMAIL_REDACTED] for details."
        );
        assert_eq!(result.redaction_count, 1);
    }

    #[test]
    fn test_currency_redaction() {
        let shield = DealShieldCore::new();
        let result = shield.sanitize_text("Revenue was $5,000,000.00 last year.");
        assert_eq!(
            result.sanitized_text,
            "Revenue was [CURRENCY_REDACTED] last year."
        );
        assert_eq!(result.redaction_count, 1);
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm_ffi {
    use super::DealShieldCore;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct WasmRedactor {
        engine: DealShieldCore,
    }

    #[wasm_bindgen]
    impl WasmRedactor {
        #[wasm_bindgen(constructor)]
        pub fn new() -> Self {
            Self { engine: DealShieldCore::new() }
        }

        #[wasm_bindgen]
        pub fn sanitize(&self, text: &str) -> String {
            self.engine.sanitize_text(text).sanitized_text
        }
    }
}
