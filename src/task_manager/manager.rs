use crate::prelude::*;
use crate::task_manager::{Task, Title};
use std::io::{Write, ErrorKind};
use std::{fs, fs::File};

#[derive(Debug)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn init() -> Result<TaskManager> {
        match fs::read_to_string("tasks.json") {
            Ok(content) => {
                match serde_json::from_str::<Vec<Task>>(&content) {
                    Ok(tasks) => return Ok(Self {
                        tasks,
                    }),
                    Err(e) => return Err(Error::SerdeJson(e)),
                }
            },
            Err(e) => {
                match e.kind() {
                    ErrorKind::NotFound => {
                        return Ok(Self {
                            tasks: vec![Task::default()],
                        });
                    },
                    _ => return Err(Error::IO(e)),
                }
            }
        }
    }
    pub fn add_task(&mut self, title: Title) {
        self.tasks.push(Task::new(title));
    }
    pub fn remove_task(&mut self, index: usize) -> Result<()> {
        if let None = self.tasks.get(index) {
            return Err(Error::Static("The index is undefined".to_string()));
        } else {
            self.tasks.remove(index);
            return Ok(());
        }
    }
    pub fn edit_task(&mut self, index: usize, title: Title) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(index) {
            task.edit(title);
            return Ok(());
        } else {
            return Err(Error::Static("The index is undefined".to_string()));
        }
    }
    pub fn mark_task(&mut self, index: usize) -> Result<()> {
        if let Some(task) = self.tasks.get_mut(index) {
            task.mark();
            return Ok(());
        } else {
            return Err(Error::Static("The index is undefined".to_string()));
        }
    }
    pub fn print_tasks(&self) {
        for (i, e) in self.tasks.iter().enumerate() {
            println!("------------------------------------------");
            println!("ID        : {}", i);
            println!("Title     : {}", e.title().value());
            println!("Completed : {}", e.completed());
            println!("------------------------------------------");
        }
    }
    pub fn close(&self) -> Result<()> {
        let mut file = File::create("tasks.json")?;
        let content = serde_json::to_string(&self.tasks)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
