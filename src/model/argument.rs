use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "CLI tool to track things placed in ToDo")]
pub struct Argument {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// list all tasks ToDo
    List {
        #[arg(short, long)]
        list: bool,
    },

    /// Add a new ToDo task
    Add {
        #[arg(short, long)]
        add: bool,

        // Title for the task
        #[arg(short, long)]
        title: String,

        // Additional description for the task
        #[arg(short, long)]
        description: String,
    },

    /// Delete existing ToDo task
    Delete {
        #[arg(short, long)]
        delete: bool,

        // Id of the task to be deleted
        #[arg(short, long)]
        id: String,
    },
}
