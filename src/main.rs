use std::{fs::File, path::Path};

use crate::model::task::Task;
mod model;
fn main() {
    let json_file_path = Path::new(".tasks.json");
    let file = File::open(json_file_path).expect("Failed to open .tasks.json file");

    let tasks: Vec<Task> = serde_json::from_reader(file).expect("Failed to parse JSON");

    for task in tasks {
        println!("{:?}", task);
    }
}
