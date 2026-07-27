//! Real token counts via the Anthropic API — the fix for the spike's
//! `tiktoken` numbers, which are for the wrong tokenizer and undercount
//! Claude worst on exactly the box-drawing and code-like content a terminal
//! dump is made of.
//!
//! Never used for asking a model a question: that path is deliberately
//! `claude -p` only (see `bench.rs`), because those two concerns need
//! different auth and different cost profiles and mixing them was how the
//! spike's numbers went wrong in the first place.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const MODEL: &str = "claude-opus-5";
const ENDPOINT: &str = "https://api.anthropic.com/v1/messages/count_tokens";
const API_VERSION: &str = "2023-06-01";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenCache {
    /// Keyed by sha256(model + "\0" + content), so a cache built for one
    /// model is never silently misread as valid for another.
    #[serde(default)]
    entries: BTreeMap<String, usize>,
}

fn cache_path(root: &Path) -> PathBuf {
    root.join("results").join("tokens.json")
}

fn hash_key(model: &str, content: &str) -> String {
    let mut h = Sha256::new();
    h.update(model.as_bytes());
    h.update(b"\0");
    h.update(content.as_bytes());
    format!("{:x}", h.finalize())
}

impl TokenCache {
    pub fn load(root: &Path) -> Result<TokenCache> {
        let p = cache_path(root);
        if !p.is_file() {
            return Ok(TokenCache::default());
        }
        let raw =
            std::fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
        serde_json::from_str(&raw).with_context(|| format!("parsing {}", p.display()))
    }

    pub fn save(&self, root: &Path) -> Result<()> {
        let p = cache_path(root);
        if let Some(dir) = p.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let mut json = serde_json::to_string_pretty(self)?;
        json.push('\n');
        std::fs::write(&p, json).with_context(|| format!("writing {}", p.display()))?;
        Ok(())
    }

    /// Real token count for `content`, or `None` if it's not cached and no
    /// `ANTHROPIC_API_KEY` is available to ask for it.
    ///
    /// A cache hit costs nothing and needs no key — that's the point: once a
    /// fixture's encodings are counted once and committed, `xtask report`
    /// runs offline for everyone after.
    pub fn get_or_count(&mut self, content: &str) -> Result<Option<usize>> {
        let key = hash_key(MODEL, content);
        if let Some(&n) = self.entries.get(&key) {
            return Ok(Some(n));
        }
        let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") else {
            return Ok(None);
        };
        let n = count_tokens_live(&api_key, content)?;
        self.entries.insert(key, n);
        Ok(Some(n))
    }
}

fn count_tokens_live(api_key: &str, content: &str) -> Result<usize> {
    let client = reqwest::blocking::Client::new();
    let body = serde_json::json!({
        "model": MODEL,
        "messages": [{"role": "user", "content": content}],
    });
    let resp = client
        .post(ENDPOINT)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .context("calling count_tokens")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().unwrap_or_default();
        anyhow::bail!("count_tokens returned {status}: {text}");
    }

    #[derive(Deserialize)]
    struct CountResponse {
        input_tokens: usize,
    }
    let parsed: CountResponse = resp.json().context("parsing count_tokens response")?;
    Ok(parsed.input_tokens)
}
