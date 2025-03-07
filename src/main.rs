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
enum Template {
    /// customizable QEMU-based image based on ebcl packages and crinit
    Ebcl,
    /// demo application and QEMU-based image based on ebclfsa and eb-hv
    Ebclfsa,
    /// empty project with a devcontainer
    Scratch,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new project from EB corbos based examples
    Startproject {
        /// the starting point for your project
        #[arg(value_enum)]
        template: Template,
        /// the name of your project
        name: String,
    },
    /// Generate shell completions
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

fn start_project(project: Template, name: &str) {
    let (project_name, template) = match project {
        Template::Ebcl => ("ebcl", PROJECT_EBCL),
        Template::Ebclfsa => ("ebclfsa", PROJECT_EBCLFSA),
        Template::Scratch => ("scratch", PROJECT_SCRATCH),
    };
    println!("Starting {} project, with name {}", project_name, name);
    extract_project(template, name).expect("Failed to extract project");
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Startproject {
            template: project,
            name,
        } => start_project(*project, name),
        Commands::Completions { shell } => {
            let mut app = Cli::command();
            let bin_name = app.get_name().to_string();
            generate(shell.clone(), &mut app, bin_name, &mut std::io::stdout());
        }
    }
}
