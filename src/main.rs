use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Project {
    Ebcl,
    Ebclfsa,
}

#[derive(Subcommand)]
enum Commands {
    StartProject {
        #[arg(value_enum)]
        project: Project,

        name: String,
    },
    /// generates shell completions
    Completions {
        /// the shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::StartProject { project, name } => match project {
            Project::Ebcl => {
                println!("Starting ebcl project, with name {}", name);
            }
            Project::Ebclfsa => {
                println!("Starting ebclfsa project, with name {}", name);
            }
        },
        Commands::Completions { shell } => {
            let mut app = Cli::command();
            let bin_name = app.get_name().to_string();
            generate(shell.clone(), &mut app, bin_name, &mut std::io::stdout());
        }
    }
}
