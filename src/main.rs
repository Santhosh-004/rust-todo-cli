use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde_json::json;

use crate::model::task::Task;
mod model;
fn main() {
    let json_file_path = Path::new(".tasks.json");
    let file: File;

    if (json_file_path.exists()) == false {
        println!(".tasks.json file not found");
        println!("Initializing task manager...");

        File::create(json_file_path).expect("Failed to create .tasks.json file");

        return;
    }
    file = File::open(json_file_path).expect("Failed to open .tasks.json file");

    let mut tasks: Vec<Task> = serde_json::from_reader(file).expect("Failed to parse JSON");

    tasks.iter().for_each(|task| {
        println!("{:?}", task);
    });

    let new_task = Task {
        id: 11,
        title: String::from("Sample Task"),
        description: String::from("This is a sample task description."),
        completed: false,
    };
    println!("New Task: {:?}", new_task);

    tasks.push(new_task);

    fs::write(json_file_path, json!(tasks).to_string()).expect("Failed to write to .tasks.json file");

    tasks.iter().for_each(|task| {
        println!("{:?}", task);
    });
}
