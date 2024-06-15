mod cli;

use clap::Parser;
use cli::add_project:: add_project;

#[derive(Parser, Debug)]
#[command(version = "0.1", author = "Marco Barreto")]
struct Args {
    #[arg(short)]
    add: bool,

    #[arg(
        long,
        value_name = "DIRECTORY",
        help = "Adds a directory to the project list"
    )]
    path: String,

    #[arg(
        long,
        value_name = "PROJECT_NAME",
        help = "Adds an alias to the project"
    )]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    process_commands(args);
}

fn process_commands(args: Args) {
    let name = args.name.unwrap_or("".to_string());
    add_project(&name);
}
