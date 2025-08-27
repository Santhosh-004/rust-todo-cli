use crate::{helper::arg_parser, model::task::Task};
use serde_json::json;
use std::{
    fs::{self, File},
    path::Path,
};
mod helper;
mod model;

fn main() {
    
    arg_parser::arg_parser();
    
    let json_file_path = Path::new(".tasks.json");
    let file: File;
    let mut tasks: Vec<Task>;

    if (json_file_path.exists()) == false {
        println!(".tasks.json file not found");
        println!("Initializing task manager...");

        File::create(json_file_path).expect("Failed to create .tasks.json file");
        tasks = Vec::new();
    } else {
        file = File::open(json_file_path).expect("Failed to open .tasks.json file");
        tasks = serde_json::from_reader(file).expect("Failed to parse JSON");
    }

    if tasks.len() > 0 {
        tasks.iter().for_each(|task| {
            println!("{:?}", task);
        });
    }

    let title = "Sample Task";
    let description = "This is a sample task description.";

    let id = helper::id_generator::id_generator(title, description);

    let new_task = Task {
        id,
        title: title.to_string(),
        description: description.to_string(),
        completed: false,
    };

    tasks.push(new_task);

    fs::write(json_file_path, json!(tasks).to_string())
        .expect("Failed to write to .tasks.json file");

    tasks.iter().for_each(|task| {
        println!("{:?}", task);
    });
}
