use std::{fs, vec};

use crate::task::Task;
use crate::error::AppError;



pub fn load_tasks() -> Result<Vec<Task>, AppError> {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.tasks.json", home);

    match fs::read_to_string(&path) {
        Ok(contents) => Ok(serde_json::from_str(&contents)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(AppError::Io(e)),
    }
}

pub fn save_tasks(tasks:&Vec<Task>) -> Result<(),AppError> {
   
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.tasks.json", home);
    let serial_data = serde_json::to_string(tasks)?;
    fs::write(&path, serial_data)?;
    return Ok(());
}

pub fn delete_task(id:u32)->Result<Vec<Task>,AppError>{
    let mut tasks = load_tasks()?;
    tasks.retain(|t| t.id != id);
    save_tasks(&tasks)?;
    Ok(tasks)   
}

pub fn toggle_complete_status(id:u32)->Result<String,AppError>{
    let mut tasks = load_tasks()?;
    let mut  found = false;
    for task in tasks.iter_mut() {
        if task.id == id {
            task.done = !task.done;
            found = true;
        }
    }
    if found {
        save_tasks(&tasks).unwrap();
        return Ok(String::from("Marked success"));
    }
    return Err(AppError::TaskNotFound(id));
}