use clap::Subcommand;

pub mod project;

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Manage projects
    #[command(subcommand)]
    Project(project::Subcommand),

    /// List projects
    Projects,
}
