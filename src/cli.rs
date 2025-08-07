use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ddns-rs", about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run DDNS
    Run(RunArgs),
}
impl Default for Commands {
    fn default() -> Self {
        Self::Run(RunArgs::default())
    }
}
#[derive(Args, Default)]
pub struct RunArgs {
    /// Config Location
    #[arg(short)]
    config: Option<PathBuf>,
}
impl RunArgs {
    pub fn config(&self) -> PathBuf {
        if let Some(path) = &self.config {
            return path.to_path_buf();
        }
        //current dir
        #[cfg(windows)]
        if let Ok(dir) = std::env::current_dir() {
            return dir.join("ddns-rs.conf");
        }
        // home/.config/ddns-rs
        if let Some(dir) = std::env::home_dir() {
            return dir.join(".config").join("ddns-rs").join("ddns-rs.conf");
        }
        //panic !
        panic!("Unable to detect configuration directory, please specify with -c")
    }
}
