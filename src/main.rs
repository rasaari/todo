use clap::{Parser, Subcommand};

mod task_file;


#[derive(Parser)]
#[command(version)]
struct CliArgs {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add {
        task: String,

        #[arg(short, long)]
        depends: Option<Vec<u32>>
    },
    Edit {
        task_id: u32,

        #[arg[short, long]]
        task: Option<String>,

        #[arg(short, long)]
        depends: Option<Vec<u32>>
    },
    Remove {
        task_id: u32
    },
    Close {
        task_id: u32
    }
}

fn main() {
    let args = CliArgs::parse();

    match &args.command {
        Commands::List => task_file::list_tasks(),
        Commands::Add { task , depends} => task_file::add_task(task, depends),
        Commands::Edit { task_id, task, depends } => task_file::edit_task(task_id, task, depends),
        Commands::Remove { task_id } => task_file::remove_task(task_id),
        Commands::Close { task_id } => task_file::remove_task(task_id)
    }
}