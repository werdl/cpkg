mod pkg;

use clap::{Subcommand, Parser};
use toml::Table;

#[derive(Subcommand)]
enum Commands {
    Install {
        pkg: String,

        #[clap(short, long)]
        version: Option<String>,
    },

    Remove {
        pkg: String,
    },

    List,
}

#[derive(Parser)]
struct Options {
    #[clap(short, long, default_value = "cpkgs.toml")]
    cfg_file: String,

    #[clap(subcommand)]
    cmd: Commands,
}

fn main() {
    let opts = Options::parse();

    println!("{:?}", opts.cfg_file);

    let toml = std::fs::read_to_string(&opts.cfg_file).unwrap();
    let toml: Table = toml::from_str(&toml).unwrap();

    println!("{:?}", toml);
    
    let registry = pkg::Registry::from_toml(toml).unwrap();

    println!("{:?}", registry);
}
