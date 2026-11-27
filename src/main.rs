#![forbid(unsafe_code)]
use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "genoffice-docs", about = "SigmaOS GenOffice document editor")]
struct Cli { #[command(subcommand)] cmd: Commands }
#[derive(Subcommand)]
enum Commands { List, Status }
fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::List   => println!("genoffice-docs: no entries configured."),
        Commands::Status => println!("genoffice-docs: ready."),
    }
}
