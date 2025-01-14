use std::path::PathBuf;

use clap::Parser;

/// Generate Nix packages with hash prefetching, license detection, and more
/// https://github.com/nix-community/nix-init
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
pub struct Opts {
    /// The path or directory to output the generated file to
    pub output: Option<PathBuf>,

    /// Specify the URL
    #[arg(short, long)]
    pub url: Option<String>,

    /// Path to nixpkgs (in nix)
    #[arg(short, long)]
    pub nixpkgs: Option<String>,

    /// Specify the config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
}
