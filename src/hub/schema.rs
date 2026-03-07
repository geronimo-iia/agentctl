use serde::{Deserialize, Serialize};

// ── Skills index (skills-index.json) ─────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillsIndex {
    pub hub_id: String,
    pub generated_at: String,
    pub skills: Vec<SkillEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillEntry {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    pub git_url: String,
    pub path: String,
    pub commit: String,
    #[serde(default)]
    pub has_lifecycle: bool,
}

// ── Docs index (docs-index.json) ─────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DocsIndex {
    #[serde(rename = "type")]
    pub kind: String,
    pub version: String,
    pub entries: Vec<DocEntry>,
    pub metadata: DocsMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocEntry {
    pub title: String,
    pub summary: String,
    pub path: String,
    pub commit_hash: String,
    pub last_updated: String,
    pub status: DocStatus,
    pub read_when: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DocStatus {
    Active,
    Draft,
    Deprecated,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocsMetadata {
    pub generated_at: String,
    pub commit_hash: String,
    pub total_entries: usize,
}
