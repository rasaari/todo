use std::io::{Write, Read};
use std::fs::File;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};


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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TaskFile {
    version: i8,
    counter: u32,
    tasks: Vec<Task>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    depends: Option<Vec<u32>>
}

impl TaskFile {
    fn task_for_id(&mut self, task_id: &u32) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|x| x.id == *task_id)
    }
}

impl Task {
    fn has_dependencies(&self, task_file: &TaskFile) -> bool {
        match &self.depends {
            Some(dependencies) => {
                for task in task_file.tasks.iter() {
                    if dependencies.contains(&task.id) {
                        return true;
                    }
                }
            },
            None => return false
        }

        return false;
    }
}

fn main() {
    let args = CliArgs::parse();

    match &args.command {
        Commands::List => list_tasks(),
        Commands::Add { task , depends} => add_task(task, depends),
        Commands::Edit { task_id, task, depends } => edit_task(task_id, task, depends),
        Commands::Remove { task_id } => remove_task(task_id),
        Commands::Close { task_id } => remove_task(task_id)
    }
}

fn load_or_create_tasks() -> TaskFile {
    let path = "tasks.toml";
    match File::open(path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Read failed");
            return toml::from_str(contents.as_str()).expect("Failed to parse");
        },
        Err(_) => {
            return TaskFile { version: 1, counter: 0, tasks: Vec::new() };
        }
    }
}

fn save_tasks(tasks: &TaskFile) {
    let serialized = toml::to_string(tasks).expect("Failed to deserialize tasks");

    let mut file = std::fs::OpenOptions::new().write(true).truncate(true).create(true).open("tasks.toml").expect("Failed to open file");
    file.write_all(serialized.as_bytes()).expect("Failed to write");
    file.flush().expect("Failed to flush");
}

fn list_tasks() -> () {
    let tasks = load_or_create_tasks();

    if tasks.tasks.len() == 0 {
        println!("No tasks");
        return;
    }

    let mut tasks_copy = Vec::from(tasks.clone().tasks);
    tasks_copy.sort_by_key(|a| a.has_dependencies(&tasks));

    for task in tasks_copy.iter() {
        println!("{: <3} #{:<5} {:<8} {}", 
        if task.has_dependencies(&tasks) { "BLK" } else { " "},
        task.id, 
        match &task.depends { Some(deps) => {deps.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")}, None => {String::from("")} },
        task.description);
    }
}

fn add_task(task: &String, depends: &Option<Vec<u32>>) {
    let mut task_file = load_or_create_tasks();

    let depends_copy:Option<Vec<u32>> = match depends {
        Some(values) => Some(values.to_vec()),
        None => None
    };

    let task_entry = Task {id: task_file.counter, description: task.to_string(), depends: depends_copy};
    task_file.tasks.push(task_entry);

    task_file.counter += 1;

    save_tasks(&task_file);
}

fn edit_task(task_id: &u32, task: &Option<String>, depends: &Option<Vec<u32>>) {
    let mut task_file = load_or_create_tasks();
    
    match task_file.task_for_id(task_id) {
        Some(task_entry) => {
            match task {
                Some(task_description) => task_entry.description = task_description.to_string(),
                None => {},
            }

            match depends {
                Some(dependencies) => { task_entry.depends = Some(dependencies.clone()) },
                None => {}
            }
        },
        None => { println!("Couldn't find task for #{}", task_id) }
    }

    save_tasks(&task_file);
}

fn remove_task(task_id: &u32) {
    let mut task_file = load_or_create_tasks();
    match task_file.clone().task_for_id(task_id) {
        Some(task) => {
            let index = task_file.clone().tasks.iter().position(|x| x.id == *task_id).unwrap();
            task_file.tasks.remove(index);

            println!("Removed task #{}", task_id);

            task_file.tasks.iter_mut().for_each(|x| {
                match &mut x.depends {
                    Some(deps) => {
                        let index = deps.iter().position(|i| *i == task.id);

                        match index {
                            Some(index_val) => {
                                deps.remove(index_val);
                            },
                            None => {}
                        }
                    },
                    None => {}
                }
            });
        },
        None => { println!("Couldn't find task for #{}", task_id) }
    }

    save_tasks(&task_file);
}