use std::path::PathBuf;

use clap::{Parser, Subcommand};
use once_cell::sync::Lazy;

use crate::db::models::Powerlevel;

/// user cli settings
pub mod user;

pub static CLI: Lazy<Cli> = Lazy::new(Cli::parse);

#[derive(Parser, Clone)]
pub struct Cli {
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// start the server
    Run,
    /// subcommands to manage users
    User {
        #[command(subcommand)]
        subcommand: UserSubcommands,
        username: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum UserSubcommands {
    /// change a users power level
    SetPowerLevel {powerlevel: Powerlevel},
    /// change a users password
    ChangePassword { new_password: String },
}
