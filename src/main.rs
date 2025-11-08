use std::fs::{self, File};
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};
use serde_json;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    name: String,
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
            get_list().expect("unable to view list");
            break;
        },
        3=> {
            println!("You selected option 3");
            remove_task();
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


fn get_list() -> io::Result<()> {
    let path = Path::new("tasks.json");
    if !path.exists() {
        let tasks: Vec<Task> = Vec::new(); 
        println!("task (from 1): {:#?}", tasks);
        return Ok(())
    }
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let tasks : Vec<Task> = serde_json::from_str(& contents).unwrap_or_else(|_| vec![]);
    println!("{:#?}", tasks);
    println!("(from 2)");
    Ok(())
}

fn remove_task()-> io::Result<()>{
    println!("Enter task Id");
    let mut task_id = String::new();
    io::stdin().read_line(&mut task_id).expect("Unable to add Id");
    let parse_task_id = task_id.trim().parse::<u32>().expect("Not a valid number");

    let path = Path::new("tasks.json");
    if !path.exists() {
        println!("No available task");
        return Ok(());
    }
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;
    let tasks :Vec<Task> = serde_json::from_str(&content).unwrap_or_else(|_| vec![]);
    let new_tasks: Vec<_> = tasks.iter().filter(|task| task.id != parse_task_id).collect(); 
    println!("{:#?}", new_tasks);
    fs::write(path, serde_json::to_string_pretty(&new_tasks).unwrap())?;
    Ok(())
}

fn update_todo()-> io::Result<()>{
    println!("Enter Id of task");
    let mut user_input = String::new();
    io::stdin().read_to_string(&mut user_input).expect("Unable to read value");
    let parse_user_input = user_input.trim().parse::<u32>().unwrap();
    let path = Path::new("tasks.json");
    if !path.exists() {
        println!("Task does not exist")
    }
    let mut contents = String::new();
    File::open(path)?.read_to_string(&mut contents)?;
    let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap();
    let mut task = tasks.iter().find(|task| task.id == parse_user_input);
    if let None = task {
        println!("No task found");
        return Ok(());
    }
    println!("{:#?}", task);
    let mut user_option = String::new();
    io::stdin().read_line(&mut user_option).expect("unable to read user input");
    let selected: Option<&str> = match user_option.as_str() {
        "1" => {
            Some("1") 
        },
        "2" => {
            Some("2")
        },
        _ =>{
            None
        }
    };
    Ok(())
}

fn completed_task(task: &Task, mut tasks: Vec<Task>) -> io::Result<Vec<Task>> {
    let task_exists = tasks.iter().any(|t| t.id == task.id);
    if !task_exists {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Task not found"
        ));
    }

    tasks.iter_mut().for_each(|t| {
        if t.id == task.id {
            t.completed = true;
        }
    });
    Ok(tasks)
}

fn update_task_name(task: &Task, mut tasks: Vec<Task>) -> io::Result<Vec<Task>> {
    
    let task_exists = tasks.iter().any(|t| t.id == task.id);
    if !task_exists {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Task not found"
        ));
    }
    println!("Enter new task name:");
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name).expect("Failed to read line");
    let new_name = new_name.trim().to_string();
    tasks.iter_mut().for_each(|t| {
        if t.id == task.id {
            t.name = new_name.clone();
        }
    });
    Ok(tasks)
}