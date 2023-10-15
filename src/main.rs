use clap::Parser;

mod command;
mod config;
mod env;
mod projects;
mod state;

use crate::config::Config;
use crate::projects::ProjectData;
use command::{project, Command};
use crate::projects::ProjectDataError::ProjectNameConflict;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The command to be executed
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug)]
enum AppError {
    ConfigError(config::ConfigError),
    ProjectDataError(projects::ProjectDataError),
}

impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err)
    }
}

impl From<projects::ProjectDataError> for AppError {
    fn from(err: projects::ProjectDataError) -> Self {
        AppError::ProjectDataError(err)
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Command::Project(subcommand)) => project::run(subcommand),
        Some(Command::Projects) => match list_projects() {
            Err(err) => eprintln!("{:?}", err),
            _ => ()
        },
        None => tui(),
    };
}

fn tui() {
    match Config::read() {
        Ok(cfg) => {
            let project_data = ProjectData::read(cfg.data_dir.as_path());
            let env = env::AppEnv::new();
            let state = state::AppState::init();

            println!("Hello, world!");
            match project_data {
                Ok(proj_data) => println!("You have {} projects", proj_data.projects.len()),
                Err(err) => println!("Failed to read projects: {}", err),
            };
            println!("Your env dummy is {}", env.dummy);

            match state.project {
                Some(project) => println!("Current project: {}", project),
                None => println!("No project selected"),
            };
        }
        Err(err) => eprintln!("{}", err),
    }
}

fn list_projects() -> Result<(), AppError> {
    let cfg = Config::read()?;
    let project_data = ProjectData::read(cfg.data_dir.as_path())?;

    if project_data.projects.len() == 0 {
        println!("No projects")
    } else {
        println!("Projects:");
        project_data.projects.iter().for_each(|project| {
            println!("- {}", project.name)
        })
    }

    Ok(())
}
