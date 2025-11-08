use std::fs::{self, File};
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};
use serde_json;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    name: String,
    id: u32,
    completed: bool
}
fn main() {
    get_user_input();
}


fn get_user_input(){
    for _ in 0..3{
        println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: u32 = input.trim().parse().expect("Please enter a valid number");
    match number {
        1=> {
            println!("You selected option 1");
            add_task().expect("Failed to add task");
            break;
        },
        2=> {
            println!("You selected option 2");
            break;
        },
        3=> {
            println!("You selected option 3");
            break;
        },
        _ => println!("Invalid input, please enter 1, 2, or 3"),
    }
    
}}


fn add_task() -> io::Result<()> {
    println!("Enter a task to add:");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("Failed to read line");
    let task = task.trim().to_string();

    let path = Path::new("tasks.json");

    // If file does not exist, create a new one with an empty list
    if !path.exists() {
        fs::write(path, "[]")?;
    }

    // Read existing contents
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;

    // Parse existing JSON (or empty vector if invalid)
    let mut tasks: Vec<Task> = serde_json::from_str(&contents).unwrap_or_else(|_| vec![]);
    let new_task = Task {
        name: String::from(&task),
        id: tasks.len() as u32 + 1,
        completed: false,
    };
    // Add new task
    tasks.push(new_task);

    // Write updated list back to file
    fs::write(path, serde_json::to_string_pretty(&tasks).unwrap())?;

    println!("✅ Task added successfully!");
    println!("{:#?}", tasks);
    Ok(())
}
