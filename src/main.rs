use clap::Parser;
use serde::{Deserialize};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Optional config file path
    #[clap(short, long)]
    config: Option<String>,
}


/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
struct Config {
    syslinks: Option<Vec<SysLink>>,
}

#[derive(Debug, Deserialize)]
struct SysLink {
    source: Option<String>,
    target: Option<String>,
}

fn main() {
    let args = Args::parse();
    let config = args.config.unwrap_or("slot.toml".to_string());
    let toml_str = std::fs::read_to_string(config).unwrap();
    println!("@@@@@@@@@ toml_str {:?}", toml_str);

    let decoded: toml::Value = toml::from_str(&toml_str).unwrap();
    let configs: Vec<Config> = decoded.try_into().unwrap();
    println!("{:#?}", configs);
}
