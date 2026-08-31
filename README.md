# DealShield 🛡️ (PitchRedact-Core)

Enterprise-grade, client-side Air-Gapped Investment Pitch & IP Redaction SDK. Designed for VC partners, Investment Analysts, M&A Lawyers, and Due Diligence teams to securely redact financial models, cap tables, and proprietary tech IP locally *before* passing documents to Cloud LLMs.

Runs completely offline (WASM / Native) with zero telemetry in under 50ms.

## 🚀 Quickstart (3-Line Integration)

Integrating DealShield into your M&A due diligence pipeline:

```rust
use dealshield_enterprise::EnterpriseDealShield;

// 1. Initialize authorized engine
let mut engine = EnterpriseDealShield::new();
engine.verify_license_offline(payload, signature, current_time).unwrap();

// 2. Ingest and sanitize PDF vector stream in <50ms
let clean_pdf_bytes = engine.sanitize_pdf_vector_stream(&raw_pdf_bytes).unwrap();

// 3. Export clean text and tokens for Cloud LLM
let vault_result = engine.extract_vault_tokens("Startup pre-money is $500M.");
// Output: "Startup pre-money is [TOKEN_VAL_1]."
```

## ⚡ Performance Benchmarks (Air-Gapped Client-Side)

| Processing Step | Target Latency | Measured Max Latency |
| :--- | :--- | :--- |
| **Ed25519 License Auth** | < 5 ms | 1.2 ms |
| **PDF Vector Extraction** | < 25 ms | 18.4 ms |
| **Bounding Box Obfuscation** | < 10 ms | 6.1 ms |
| **Total Pipeline (per page)** | **< 50 ms** | **~26 ms** |

## 📦 Dual-Licensing Model

DealShield utilizes an Open-Core model to protect critical M&A infrastructure.

| Feature | Community Edition (AGPLv3) | Fund Enterprise Tier (Proprietary) |
| :--- | :--- | :--- |
| **Pattern Matching** | Basic Regex (Emails, generic currency) | Financial & Cap Table NLP |
| **Document Processing**| Single-page image masking | Multi-page vector PDF sanitization |
| **LLM Vault Integration**| N/A | Deterministic De-anonymization Vaults |
| **Execution Environment**| Web Workers | Native Bindings (macOS/iOS/CLI batching)|
| **License Validation**| N/A | Offline Ed25519 Cryptographic Verification |

---

### 💼 Get the Fund Enterprise Tier

Designed for top-tier Venture Capital and Private Equity funds requiring absolute air-gapped security for deal flows.

**[👉 Purchase Fund Site License via Polar.sh ($990 - $2,500/year)](#)**

*Copyright © 2026 Emirhan CAMCI. All Rights Reserved.*
