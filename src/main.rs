use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{Shell, generate};
use flate2::read::GzDecoder;
use std::io::Cursor;
use tar::Archive;

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
    Scratch,
}

#[derive(Subcommand)]
enum Commands {
    Startproject {
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

// Embed project template in the binary
const PROJECT_EBCL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "ebcl.tar.gz"));
const PROJECT_EBCLFSA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "ebclfsa.tar.gz"));
const PROJECT_SCRATCH: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "scratch.tar.gz"));

fn extract_project(template: &[u8], name: &str) -> std::io::Result<()> {
    let tar = GzDecoder::new(Cursor::new(template));
    let mut archive = Archive::new(tar);
    archive.unpack(name)?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Startproject { project, name } => match project {
            Project::Ebcl => {
                println!("Starting ebcl project, with name {}", name);
                extract_project(PROJECT_EBCL, name).expect("Failed to extract project");
            }
            Project::Ebclfsa => {
                println!("Starting ebclfsa project, with name {}", name);
                extract_project(PROJECT_EBCLFSA, name).expect("Failed to extract project");
            }
            Project::Scratch => {
                println!("Starting scratch project, with name {}", name);
                extract_project(PROJECT_SCRATCH, name).expect("Failed to extract project");
            }
        },
        Commands::Completions { shell } => {
            let mut app = Cli::command();
            let bin_name = app.get_name().to_string();
            generate(shell.clone(), &mut app, bin_name, &mut std::io::stdout());
        }
    }
}
