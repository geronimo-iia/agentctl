use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

/// CLI for agent hub validation, index generation, and skill management.
#[derive(Parser)]
#[command(name = "agentctl", version, about, long_about = None)]
pub struct Cli {
    /// Path to the config file (default: ~/.agentctl/config.json).
    #[arg(long, global = true)]
    pub config: Option<PathBuf>,

    /// Suppress all output; implies --yes.
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Auto-approve all lifecycle steps without prompting.
    #[arg(long, short = 'y', global = true)]
    pub yes: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage agent hubs (validate, generate index, registry).
    Hub {
        #[command(subcommand)]
        action: HubAction,
    },
    /// Manage installed skills.
    Skill {
        #[command(subcommand)]
        action: SkillAction,
    },
}

#[derive(Subcommand)]
pub enum HubAction {
    /// Validate all files in a hub directory against the schema.
    Validate {
        /// Hub type to validate.
        #[arg(long, value_enum)]
        r#type: HubType,

        /// Path to the hub directory.
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },

    /// Generate index.json for a hub directory.
    Generate {
        /// Hub type to generate index for.
        #[arg(long, value_enum)]
        r#type: HubType,

        /// Path to the hub directory.
        #[arg(long, default_value = ".")]
        path: PathBuf,

        /// Output path for the generated index.json.
        #[arg(long, default_value = "index.json")]
        output: PathBuf,

        /// Hub identifier written into the skills index (skills hubs only).
        #[arg(long, default_value = "default")]
        hub_id: String,
    },

    /// Register a hub in the local config.
    Add {
        /// Hub kind (skills or docs).
        #[arg(long, value_enum)]
        r#type: HubType,

        /// Unique hub identifier.
        id: String,

        /// URL of the hub index.json.
        index_url: String,

        /// Optional git repository URL.
        #[arg(long)]
        git_url: Option<String>,
    },

    /// List registered hubs.
    List,

    /// Remove a hub from the local config.
    Remove {
        /// Hub identifier to remove.
        id: String,
    },

    /// Enable a registered hub.
    Enable {
        /// Hub identifier to enable.
        id: String,
    },

    /// Disable a registered hub.
    Disable {
        /// Hub identifier to disable.
        id: String,
    },

    /// Refresh the cached index for one or all enabled hubs.
    Refresh {
        /// Hub identifier to refresh. Refreshes all enabled hubs if omitted.
        id: Option<String>,
    },
}

/// Hub content type.
#[derive(Clone, ValueEnum)]
pub enum HubType {
    /// Skill hub containing SKILL.md files.
    Skills,
    /// Documentation hub containing Markdown files.
    Docs,
}

#[derive(Subcommand)]
pub enum SkillAction {
    /// Install a skill from a registered hub.
    Install {
        /// Skill name.
        name: String,
        /// Hub identifier to install from.
        #[arg(long)]
        hub: Option<String>,
        /// Install mode (sets install path to ~/.agent/skills-{mode}/).
        #[arg(long)]
        mode: Option<String>,
    },
    /// List installed skills.
    List,
    /// Remove an installed skill.
    Remove {
        /// Skill name.
        name: String,
        /// Hub identifier the skill was installed from.
        #[arg(long)]
        hub: String,
    },
    /// Update an installed skill to the latest version.
    Update {
        /// Skill name. Updates all skills if omitted.
        name: Option<String>,
        /// Hub identifier.
        #[arg(long)]
        hub: Option<String>,
    },
}
