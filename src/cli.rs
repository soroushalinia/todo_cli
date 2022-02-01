use crate::tasks;
use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: Option<String>,
        #[clap(short = 'd')]
        date: Option<String>,
    },
    Ls,
}

pub fn parse_args() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { name, date } => {
            let date_str = date.as_deref().unwrap_or_else(|| "");
            let name_str = match name.as_deref() {
                Some(value) => value,
                None => {
                    eprintln!("error: no name was provided.");
                    exit(1);
                }
            };
            match tasks::new_task(name_str.to_string(), date_str.to_string()) {
                Ok(value) => println!("Task '{}' was created.", value),
                Err(err) => eprintln!("There was an error creating new task: {}", err),
            }
        }
        Commands::Ls => {}
    }
}