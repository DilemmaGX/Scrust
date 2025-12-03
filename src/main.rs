mod ast;
mod compiler;
mod config;
mod parser;
mod sb3;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use config::ScrustConfig;
use nom::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use zip::write::FileOptions;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project
    Build {
        /// Path to Scrust.toml
        #[arg(short, long, default_value = "scrust.toml")]
        config: PathBuf,

        /// Output project.json for debugging
        #[arg(long, default_value_t = false)]
        debug: bool,
    },
    /// Create a new project
    Create {
        /// Project name
        name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { config, debug } => build(config, debug),
        Commands::Create { name } => create(name),
    }
}

fn format_parse_error(e: nom::Err<Error<&str>>, src: &str) -> String {
    match e {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            let offset = e.input.as_ptr() as usize - src.as_ptr() as usize;
            let line_num = src[..offset].lines().count().max(1);
            let line_content = src.lines().nth(line_num - 1).unwrap_or("");
            format!(
                "Parse error at line {}:\n  {}\n  {}",
                line_num,
                line_content.trim(),
                "^".red().bold()
            )
        }
        nom::Err::Incomplete(_) => "Parse incomplete".red().to_string(),
    }
}

fn build(config_path: PathBuf, debug: bool) -> Result<()> {
    println!("{}", "Building project...".blue().bold());
    let config_str = fs::read_to_string(&config_path)?;
    let config: ScrustConfig = toml::from_str(&config_str)?;
    let config_dir = config_path.parent().unwrap();

    let mut targets = Vec::new();
    let mut assets_to_pack = Vec::new();

    // Compile Stage
    let stage_path = if config.stage.path.is_absolute() {
        config.stage.path.clone()
    } else {
        config_dir.join(&config.stage.path)
    };

    let stage_src = fs::read_to_string(&stage_path)?;
    let (rest, mut stage_ast) = parser::parse_program(&stage_src)
        .map_err(|e| anyhow::anyhow!("{}", format_parse_error(e, &stage_src)))?;
    if !rest.trim().is_empty() {
        println!(
            "{}",
            format!(
                "Warning: Stage parsing stopped early. Remaining: {:.50}...",
                rest
            )
            .yellow()
        );
    }

    // Pre-load sprites to extract public variables
    let mut sprite_data = Vec::new();
    if let Some(sprites) = &config.sprite {
        println!(
            "{}",
            format!("Found {} sprites in config", sprites.len()).blue()
        );
        for sprite in sprites {
            println!("{}", format!("Processing sprite: {:?}", sprite.name).cyan());
            let sprite_path = if sprite.path.is_absolute() {
                sprite.path.clone()
            } else {
                config_dir.join(&sprite.path)
            };
            let src = fs::read_to_string(&sprite_path)?;
            let (rest, ast) = parser::parse_program(&src).map_err(|e| {
                anyhow::anyhow!(
                    "In sprite '{}': {}",
                    sprite.name.as_deref().unwrap_or("unknown"),
                    format_parse_error(e, &src)
                )
            })?;
            if !rest.trim().is_empty() {
                println!(
                    "{}",
                    format!(
                        "Warning: Sprite {} parsing stopped early. Remaining: {:.50}...",
                        sprite.name.as_deref().unwrap_or("unknown"),
                        rest
                    )
                    .yellow()
                );
            }

            // Extract public variables and add to stage_ast
            for item in &ast.items {
                if let ast::Item::Variable(decl) = item {
                    if decl.visibility == ast::Visibility::Public {
                        // Check if already exists in stage
                        let exists = stage_ast.items.iter().any(|i| {
                            if let ast::Item::Variable(v) = i {
                                v.name == decl.name
                            } else {
                                false
                            }
                        });
                        if !exists {
                            stage_ast.items.push(ast::Item::Variable(decl.clone()));
                        }
                    }
                }
            }
            sprite_data.push((sprite, ast));
        }
    }

    let (stage_target, stage_assets) =
        compiler::compile_target(&stage_ast, true, None, None, config_dir);

    for (path, filename) in stage_assets {
        assets_to_pack.push((path, filename));
    }

    let global_vars = stage_target.variables.clone();
    let global_lists = stage_target.lists.clone();
    targets.push(stage_target);

    for (sprite, ast) in sprite_data {
        let (mut target, sprite_assets) = compiler::compile_target(
            &ast,
            false,
            Some(&global_vars),
            Some(&global_lists),
            config_dir,
        );
        target.name = sprite.name.clone().unwrap_or("Sprite".to_string());

        for (path, filename) in sprite_assets {
            assets_to_pack.push((path, filename));
        }
        targets.push(target);
    }

    let project = sb3::Sb3Project {
        targets,
        monitors: vec![],
        extensions: config.project.extensions.unwrap_or_default(),
        meta: sb3::Meta {
            semver: "3.0.0".to_string(),
            vm: "0.2.0".to_string(),
            agent: "Scrust 0.1.4".to_string(),
        },
    };

    // Package .sb3
    // Determine output directory
    let output_dir = if config.project.output.extension().is_some() {
        // If output has extension, treat it as file path and get parent
        if config.project.output.is_absolute() {
            config.project.output.parent().unwrap().to_path_buf()
        } else {
            config_path
                .parent()
                .unwrap()
                .join(config.project.output.parent().unwrap())
        }
    } else {
        // Treat as directory
        if config.project.output.is_absolute() {
            config.project.output.clone()
        } else {
            config_path.parent().unwrap().join(&config.project.output)
        }
    };

    fs::create_dir_all(&output_dir)?;

    let safe_name = config.project.name.replace(['/', '\\'], "_");
    let output_filename = format!("{}.sb3", safe_name);
    let output_path = output_dir.join(&output_filename);

    if debug {
        let debug_path = output_dir.join("project.json");
        let debug_file = fs::File::create(&debug_path)?;
        serde_json::to_writer_pretty(debug_file, &project)?;
        println!(
            "{}",
            format!("Debug output written to {:?}", debug_path).dimmed()
        );
    }

    let file = fs::File::create(&output_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file("project.json", options)?;
    serde_json::to_writer(&mut zip, &project)?;

    for (path, filename) in assets_to_pack {
        let content = fs::read(&path).context(format!("Failed to read asset {:?}", path))?;
        zip.start_file(filename, options)?;
        zip.write_all(&content)?;
    }

    zip.finish()?;

    println!(
        "{}",
        format!("Build complete: {:?}", output_path).green().bold()
    );
    Ok(())
}

fn create(name: String) -> Result<()> {
    let root = PathBuf::from(&name);
    if root.exists() {
        return Err(anyhow::anyhow!("Directory '{}' already exists", name));
    }

    println!(
        "{}",
        format!("Creating project '{}'...", name).blue().bold()
    );

    fs::create_dir_all(root.join("src"))?;
    fs::create_dir_all(root.join("assets"))?;
    // dist directory will be created on build

    // Extract just the folder name for the project name
    let project_name = root.file_name().and_then(|n| n.to_str()).unwrap_or(&name);

    // scrust.toml
    let config_content = format!(
        r#"[project]
name = "{}"
output = "dist/project.sb3"

[stage]
path = "src/stage.sr"

[[sprite]]
name = "Sprite1"
path = "src/sprite.sr"
"#,
        project_name
    );
    fs::write(root.join("scrust.toml"), config_content)?;

    // src/stage.sr
    let stage_content = r#"costume "backdrop1" "assets/stage.svg";
"#;
    fs::write(root.join("src").join("stage.sr"), stage_content)?;

    // src/sprite.sr
    let sprite_content = r#"costume "costume1" "assets/sprite.svg";

#[on_flag_clicked]
fn start() {
    say("Hello, Scrust!");
}
"#;
    fs::write(root.join("src").join("sprite.sr"), sprite_content)?;

    // assets/stage.svg
    let stage_svg = r##"<svg version="1.1" width="480" height="360" xmlns="http://www.w3.org/2000/svg"><rect width="480" height="360" fill="#ffffff"/></svg>"##;
    fs::write(root.join("assets").join("stage.svg"), stage_svg)?;

    // assets/sprite.svg
    let sprite_svg = include_str!("../assets/logo-64.svg");
    fs::write(root.join("assets").join("sprite.svg"), sprite_svg)?;

    println!("{}", "Project created successfully!".green().bold());
    println!("cd {}\ncargo run -- build", name);

    Ok(())
}
