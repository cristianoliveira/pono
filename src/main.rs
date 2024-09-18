use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use std::os::unix::fs::symlink;
use std::path::PathBuf;

/// pono - pack and organize symlinks once
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Commands available
    #[command(subcommand)]
    command: Commands,

    /// Optional config file path (default: pono.toml)
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Enables all or a space-separated list of ponos
    Enable {
        /// Optional list of ponos to enable (default: all)
        ponos: Option<Vec<String>>,
    },
    /// Disable all or a space-separated list of ponos
    Disable {
        /// Optional list of ponos to disable (default: all)
        ponos: Option<Vec<String>>,
    },
    /// Display the status of all ponos
    Status {
        /// Optional list of ponos to check (default: all)
        ponos: Option<Vec<String>>,
    },
    /// List all ponos in the configuration
    List,
}

// Configuration file format
#[derive(Debug, Deserialize)]
struct Configuration {
    ponos: HashMap<String, PonoDefinition>,
}

#[derive(Debug, Deserialize)]
struct PonoDefinition {
    source: String,
    target: String,
}

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
    let args = Args::parse();

    // Validate all ponos before performing filesystem operations
    match &args.command {
        Commands::Enable { ponos } | Commands::Disable { ponos } => {
            let config = load_config(&args.config);
            for pkg_name in ponos_to_manipulate(&config, &ponos) {
                let pono_definition = config.ponos.get(&pkg_name).unwrap();
                match validate_package(&pono_definition) {
                    Ok(_) => (),
                    Err(PonoError::TargetAlreadyExists(err)) => {
                        if let Commands::Enable { .. } = args.command {
                            print!("{}", RED);
                            println!("Invalid ponos: {}", pkg_name);
                            println!("Reason: {}", err);
                            print!("{}", RESET);
                            std::process::exit(1);
                        }
                    }
                    Err(err) => {
                        print!("{}", RED);
                        println!("Invalid pono: {}", pkg_name);
                        println!("Reason: {}", err);
                        print!("{}", RESET);
                        std::process::exit(1);
                    }
                }
            }
        }
        _ => (),
    }

    match args.command {
        Commands::Enable { ponos } => {
            // Commands with side effects
            let config = load_config(&args.config);
            println!("Linking ponos");
            for pkg_name in ponos_to_manipulate(&config, &ponos) {
                let pono_definition = config.ponos.get(&pkg_name).unwrap();

                println!(
                    "{}  {} -> {} (linking)",
                    pkg_name, pono_definition.source, pono_definition.target
                );
                let src_path = path(&pono_definition.source);

                match symlink(&src_path, &pono_definition.target) {
                    Ok(_) => {
                        println!(
                            "{}  {}: {} (new link)",
                            GREEN, pkg_name, pono_definition.target
                        )
                    }
                    Err(err) => {
                        println!("{}Pono link failed reason: {}", RED, err);
                        std::process::exit(1);
                    }
                };
                print!("{}", RESET);
            }
        }
        Commands::Disable { ponos } => {
            let config = load_config(&args.config);
            for pkg_name in ponos_to_manipulate(&config, &ponos) {
                let pono_definition = config.ponos.get(&pkg_name).unwrap();
                match std::fs::remove_file(&pono_definition.target) {
                    Ok(_) => println!("Unlinked pono: {}", pkg_name),
                    Err(err) => {
                        print!("{}", RED);
                        println!("Failed to unlink pono: {}", err);
                        print!("{}", RESET);
                        std::process::exit(1);
                    }
                }
            }
        }

        // Commands without side effects
        Commands::Status { ponos } => {
            println!("Status:");
            let mut has_error = false;
            let config = load_config(&args.config);
            for pkg_name in ponos_to_manipulate(&config, &ponos) {
                let pono_definition = config.ponos.get(&pkg_name).unwrap();

                match check_package(&pono_definition) {
                    Ok(_) => {
                        print!("{}", GREEN);
                        println!("  {} {} (linked)", pkg_name, pono_definition.target);
                    }
                    Err(err) => {
                        print!("{}", RED);
                        println!("  {} {} (broken)", pkg_name, pono_definition.target);
                        println!("  Reason: {}", err);
                        has_error = true;
                    }
                };
                print!("{}", RESET);
            }

            if has_error {
                std::process::exit(1);
            }
        }
        Commands::List => {
            let config = load_config(&args.config);
            println!("Ponos:");
            for (package, pono_definition) in config.ponos.iter() {
                println!("  {}: {}", package, pono_definition.source);
            }
        }
    }
}

enum PonoError {
    NotFound(String),
    NotSymlink(String),
    TargetAlreadyExists(String),
    LinkMismatch(String),
    Unhandled(String),
}
impl Display for PonoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PonoError::NotFound(msg) => write!(f, "(not-found) {}", msg),
            PonoError::NotSymlink(msg) => write!(f, "(non-pono_definition) {}", msg),
            PonoError::LinkMismatch(msg) => write!(f, "(link-mismatch) {}", msg),
            PonoError::TargetAlreadyExists(msg) => write!(f, "(not-available) {}", msg),
            PonoError::Unhandled(msg) => write!(f, "{}", msg),
        }
    }
}

fn load_config(config_arg: &Option<String>) -> Configuration {
    let config = config_arg.clone().unwrap_or("pono.toml".to_string());
    let config_path = path(&config);
    let toml_content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(err) => {
            println!("Failed to read the {} file", config);
            println!("Reason: {}", err);
            println!("Debugging:");
            println!(" - Check if file exists and is accessible (using ls -la)");
            std::process::exit(1);
        }
    };

    let maybe_config: Option<Configuration> = match toml::from_str(&toml_content) {
        Ok(config) => Some(config),
        Err(err) => {
            println!("Invalid pono configuration file");
            println!("Reason: {}", err);
            println!("Debugging:");
            println!(" - Check if the file is a valid TOML file");
            println!(" - Check if the file has the correct format");
            None
        }
    };

    if maybe_config.is_none() {
        std::process::exit(1);
    }

    return maybe_config.unwrap();
}

fn validate_package(package: &PonoDefinition) -> Result<(), PonoError> {
    let sln_path = path(&package.target);
    let src_path = path(&package.source);

    // check if source exists
    if !std::path::Path::new(&src_path).exists() {
        return Err(PonoError::NotFound(format!(
            "Pono source does not exist: {} ",
            src_path
        )));
    }

    if std::path::Path::new(&sln_path).exists() {
        let sln_metadata = match std::fs::symlink_metadata(&sln_path) {
            Ok(metadata) => metadata,
            Err(err) => {
                return Err(PonoError::Unhandled(format!(
                    "Target isn't accessible: {:?}",
                    err
                )))
            }
        };

        if !sln_metadata.file_type().is_symlink() {
            let type_str = if sln_metadata.is_dir() {
                "directory"
            } else if sln_metadata.is_file() {
                "file"
            } else {
                "unknown"
            };

            return Err(PonoError::TargetAlreadyExists(format!(
                "(not-available) Target path '{}' already exists and is a {}.",
                &package.target, type_str
            )));
        }
        return Err(PonoError::TargetAlreadyExists(format!(
            "(not-avaiable) Pono target already exists: {}",
            src_path
        )));
    }

    Ok(())
}

fn check_package(package: &PonoDefinition) -> Result<(), PonoError> {
    let sln_path = path(&package.target);
    let src_path = path(&package.source);

    let sln_metadata = match std::fs::symlink_metadata(&sln_path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(PonoError::NotFound(format!(
                "Failed to read metadata: {:?}",
                err
            )))
        }
    };

    if !sln_metadata.file_type().is_symlink() {
        let type_str = if sln_metadata.is_dir() {
            "directory"
        } else if sln_metadata.is_file() {
            "file"
        } else {
            "unknown"
        };

        return Err(PonoError::NotSymlink(format!(
            "Target path '{}' already exists and is a {}.",
            &package.target, type_str
        )));
    }

    let target_path = std::fs::read_link(sln_path).expect("Failed to read link");

    if !target_path.exists() {
        return Err(PonoError::NotFound(format!(
            "Target path does not exist: {}",
            target_path.display()
        )));
    }

    // Get the metadata of the target (source) file following the symlink
    let target_metatada = match std::fs::metadata(&target_path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(PonoError::Unhandled(format!(
                "Failed to read target metadata: {}",
                err
            )))
        }
    };
    let src_metadata = match std::fs::metadata(src_path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(PonoError::Unhandled(format!(
                "Failed to read source metadata: {}",
                err
            )))
        }
    };

    // Compare inode and device numbers to check if they point to the same file
    if target_metatada.len() == src_metadata.len() {
        return Ok(());
    }

    Err(PonoError::LinkMismatch(format!(
        "Package link mismatch between target '{}' and source '{}'",
        package.target, package.source
    )))
}

fn path(path: &str) -> String {
    if path.starts_with("~") {
        return shellexpand::tilde(path).into_owned();
    }

    if path.contains("$") {
        return shellexpand::env(path)
            .expect("Failed to expand environment variables")
            .into_owned();
    }

    let pathbuf = PathBuf::from(path);

    let absolute_path = if pathbuf.is_absolute() {
        pathbuf.to_path_buf()
    } else {
        env::current_dir()
            .expect("Failed to get current directory")
            .join(pathbuf)
    };

    absolute_path.to_string_lossy().to_string()
}

fn ponos_to_manipulate(config: &Configuration, ponos: &Option<Vec<String>>) -> Vec<String> {
    config
        .ponos
        .keys()
        .filter(|p| contain_package(ponos, p))
        .map(|s| s.to_string())
        .collect()
}

fn contain_package(ponos: &Option<Vec<String>>, package: &str) -> bool {
    if ponos.is_none() {
        return true;
    }

    if let Some(ref pckgs) = ponos {
        if pckgs.contains(&package.to_string()) {
            return true;
        }
    }

    false
}
