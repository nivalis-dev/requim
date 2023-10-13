use clap;

#[derive(Clone, Debug, clap::Subcommand)]
pub enum Subcommand {
    New(NewArgs),
}

#[derive(Clone, Debug, clap::Args)]
pub struct NewArgs {
    pub name: String,
}

pub fn run(subcommand: Subcommand) {
    match subcommand {
        Subcommand::New(project_name) => create_project(project_name)
    };
}

fn create_project(args: NewArgs) {
    println!("Not implemented: create project \"{}\"", args.name);
}
