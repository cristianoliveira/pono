use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fmt::{Display, Formatter};
use std::os::unix::fs::symlink;
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Commands available
    #[command(subcommand)]
    command: Option<Commands>,

    /// Optional config file path (default: slot.toml)
    #[clap(short, long)]
    config: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Link all or a space-separated list of packages
    Link {
        /// Optional list of packages to link (default: all)
        packages: Option<Vec<String>>,
    },
    /// Remove the link of all or a space-separated list of packages
    Unlink {
        /// Optional list of packages to link (default: all)
        packages: Option<Vec<String>>,
    },
    /// Display the status of all packages
    Status {
        /// Optional list of packages to check (default: all)
        packages: Option<Vec<String>>,
    },
    /// List all packages in the configuration
    List,
}

// Configuration file format
#[derive(Debug, Deserialize)]
struct Configuration {
    packages: HashMap<String, SysLink>,
}

#[derive(Debug, Deserialize)]
struct SysLink {
    source: String,
    target: String,
}

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
    let args = Args::parse();
    let config = args.config.unwrap_or("slot.toml".to_string());
    let config_path = path(&config);
    let toml_str = std::fs::read_to_string(&config_path)
        .expect(format!("Failed to read configuration file: {}", config).as_str());

    let config: Configuration = toml::from_str(&toml_str)
        .expect(format!("Failed to parse configuration file: {}", config).as_str());

    match args.command.unwrap() {
        Commands::Link { packages } => {
            println!("Linking packages");
            for (pkg_name, syslink) in config.packages.iter() {
                if contain_package(&packages, &pkg_name) {
                    continue;
                }

                match check_package(&syslink) {
                    Ok(_) => {
                        print!("{}", GREEN);
                        println!("  {}: {} (linked)", pkg_name, syslink.target)
                    }
                    Err(SlotError::NotFound(_)) => {
                        let src_path = path(&syslink.source);
                        match symlink(&src_path, &syslink.target) {
                            Ok(_) => {
                                println!("{}  {}: {} (new link)", GREEN, pkg_name, syslink.target)
                            }
                            Err(err) => {
                                println!("{}Package link failed: {}", RED, err)
                            }
                        }
                    }
                    Err(err) => {
                        print!("{}", RED);
                        println!("  {} {} (broken)", pkg_name, syslink.target);
                        println!("  Reason: {}", err);
                    }
                };
                print!("{}", RESET);
            }
        }
        Commands::Unlink { packages } => {
            for (pkg_name, syslink) in config.packages.iter() {
                if contain_package(&packages, &pkg_name) {
                    continue;
                }
                match std::fs::remove_file(&syslink.target) {
                    Ok(_) => println!("Unlinked package: {}", pkg_name),
                    Err(err) => {
                        print!("{}", RED);
                        println!("Failed to unlink package: {}", err);
                        print!("{}", RESET);
                    }
                }
            }
        }
        Commands::List => {
            println!("Packages:");
            for (package, syslink) in config.packages.iter() {
                println!("  {}: {}", package, syslink.source);
            }
        }
        Commands::Status { packages } => {
            println!("Status:");
            let mut has_error = false;
            for (pkg_name, syslink) in config.packages.iter() {
                if contain_package(&packages, &pkg_name) {
                    continue;
                }

                match check_package(&syslink) {
                    Ok(_) => {
                        print!("{}", GREEN);
                        println!("  {} {} (linked)", pkg_name, syslink.target);
                    }
                    Err(err) => {
                        print!("{}", RED);
                        println!("  {} {} (broken)", pkg_name, syslink.target);
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
    }
}

fn contain_package(packages: &Option<Vec<String>>, package: &str) -> bool {
    if let Some(ref pckgs) = packages {
        if pckgs.contains(&package.to_string()) {
            return true;
        }
    }

    false
}

enum SlotError {
    NotFound(String),
    NotSymlink(String),
    LinkMismatch(String),
    Unhandled(String),
}
impl Display for SlotError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SlotError::NotFound(msg) => write!(f, "Not found: {}", msg),
            SlotError::NotSymlink(msg) => write!(f, "Not a symlink: {}", msg),
            SlotError::LinkMismatch(msg) => write!(f, "Link mismatch: {}", msg),
            SlotError::Unhandled(msg) => write!(f, "Unhandled error: {}", msg),
        }
    }
}

fn check_package(package: &SysLink) -> Result<(), SlotError> {
    let sln_path = path(&package.target);
    let src_path = path(&package.source);

    let sln_metadata = match std::fs::symlink_metadata(&sln_path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(SlotError::NotFound(format!(
                "Failed to read metadata: {:?}",
                err
            )))
        }
    };

    if !sln_metadata.file_type().is_symlink() {
        return Err(SlotError::NotSymlink(format!(
            "Path is not a symlink: {}",
            &package.target
        )));
    }

    let target_path = std::fs::read_link(sln_path).expect("Failed to read link");

    if !target_path.exists() {
        return Err(SlotError::NotFound(format!(
            "Target path does not exist: {}",
            target_path.display()
        )));
    }

    // Get the metadata of the target (source) file following the symlink
    let target_metatada = match std::fs::metadata(&target_path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(SlotError::Unhandled(format!(
                "Failed to read target metadata: {}",
                err
            )))
        }
    };
    let src_metadata = match std::fs::metadata(src_path) {
        Ok(metadata) => metadata,
        Err(err) => {
            return Err(SlotError::Unhandled(format!(
                "Failed to read source metadata: {}",
                err
            )))
        }
    };

    // Compare inode and device numbers to check if they point to the same file
    if target_metatada.len() == src_metadata.len() {
        return Ok(());
    }

    Err(SlotError::LinkMismatch(format!(
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
