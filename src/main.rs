use clap::Parser;

mod config;
mod env;
mod state;
mod command;

use command::{Command, project};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to be executed
    #[command(subcommand)]
    command: Option<Command>,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Project(subcommand)) => project::run(subcommand),
        Some(Command::Projects) => list_projects(),
        None => tui(),
    };
}

fn tui() {
    let cfg = config::Config::new();
    let env = env::AppEnv::new();
    let state = state::AppState::init();

    println!("Hello, world!");
    println!("You have {} projects", cfg.projects.len());
    println!("Your env dummy is {}", env.dummy);

    match state.project {
        Some(project) => println!("Current project: {}", project),
        None => println!("No project selected")
    };
}

fn list_projects() {
    println!("You have no projects")
}
