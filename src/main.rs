mod cli;
mod config;
mod hub;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command, HubAction, HubType};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let cfg_path = cli.config.unwrap_or_else(config::config_path);

    match cli.command {
        Command::Hub { action } => match action {
            HubAction::Validate { r#type, path } => {
                let path = path.canonicalize()?;
                let result = match r#type {
                    HubType::Skills => {
                        eprintln!("Validating skills hub at {}", path.display());
                        hub::validate::validate_skills_hub(&path)?
                    }
                    HubType::Docs => {
                        eprintln!("Validating docs hub at {}", path.display());
                        hub::validate::validate_docs_hub(&path)?
                    }
                };

                if result.is_valid() {
                    println!("✓ Validation passed");
                } else {
                    for err in &result.errors {
                        eprintln!("  ✗ {err}");
                    }
                    eprintln!("✗ Validation failed ({} error(s))", result.errors.len());
                    std::process::exit(1);
                }
            }

            HubAction::Generate {
                r#type,
                path,
                output,
                hub_id,
            } => {
                let path = path.canonicalize()?;
                eprintln!(
                    "Generating {} index from {} to {}",
                    match r#type {
                        HubType::Skills => "skills",
                        HubType::Docs => "docs",
                    },
                    path.display(),
                    output.display()
                );

                let json = match r#type {
                    HubType::Skills => {
                        let index = hub::generate::generate_skills_index(&path, &hub_id)?;
                        serde_json::to_string_pretty(&index)?
                    }
                    HubType::Docs => {
                        let index = hub::generate::generate_docs_index(&path)?;
                        serde_json::to_string_pretty(&index)?
                    }
                };

                std::fs::write(&output, json)?;
                println!("✓ Generated {}", output.display());
            }

            HubAction::Add {
                r#type,
                id,
                index_url,
                git_url,
            } => {
                let kind = hub_kind(r#type);
                hub::registry::add(&cfg_path, kind, &id, &index_url, git_url.as_deref())?;
                println!("✓ Added hub '{id}'");
            }

            HubAction::List => {
                let cfg = config::Config::load_from(&cfg_path)?;
                println!("Skill hubs:");
                for h in &cfg.skill_hubs {
                    println!(
                        "  {} [{}] {}",
                        h.id,
                        if h.enabled { "enabled" } else { "disabled" },
                        h.index_url
                    );
                }
                println!("Doc hubs:");
                for h in &cfg.doc_hubs {
                    println!(
                        "  {} [{}] {}",
                        h.id,
                        if h.enabled { "enabled" } else { "disabled" },
                        h.index_url
                    );
                }
            }

            HubAction::Remove { id } => {
                hub::registry::remove(&cfg_path, &id)?;
                println!("✓ Removed hub '{id}'");
            }

            HubAction::Enable { id } => {
                hub::registry::set_enabled(&cfg_path, &id, true)?;
                println!("✓ Enabled hub '{id}'");
            }

            HubAction::Disable { id } => {
                hub::registry::set_enabled(&cfg_path, &id, false)?;
                println!("✓ Disabled hub '{id}'");
            }

            HubAction::Refresh { id } => match id {
                Some(id) => {
                    hub::registry::refresh_one(&cfg_path, &id)?;
                    println!("✓ Refreshed hub '{id}'");
                }
                None => {
                    hub::registry::refresh_all(&cfg_path)?;
                    println!("✓ Refreshed all enabled hubs");
                }
            },
        },
    }

    Ok(())
}

fn hub_kind(t: HubType) -> hub::registry::HubKind {
    match t {
        HubType::Skills => hub::registry::HubKind::Skill,
        HubType::Docs => hub::registry::HubKind::Doc,
    }
}
