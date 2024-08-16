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
                if content.is_empty() {
                    let title = Title::new("Testing")?;
                    let task = Task::new(title);

                    return Ok(Self {
                        tasks: vec![task],
                    });
                } else {
                    let tasks: Vec<Task> = serde_json::from_str(&content)?;

                    return Ok(Self {
                        tasks: tasks,
                    });
                }
            },
            Err(err) => {
                if err.kind() == ErrorKind::NotFound {
                    let title = Title::new("Testing")?;
                    let task = Task::new(title);

                    return Ok(Self {
                        tasks: vec![task],
                    });
                }

                return Err(Error::IO(err));
            },
        }
    }
    pub fn add_task(&mut self, title: Title) {
        let task = Task::new(title);
        self.tasks.push(task);
    }
    pub fn remove_task(&mut self, index: usize) -> Result<()> {
        if index > self.tasks.len() {
            return Err(Error::Static("Invalid Index".to_string()));
        }

        self.tasks.remove(index);
        Ok(())
    }
    pub fn edit_task(&mut self, index: usize, title: Title) -> Result<()> {
        match self.tasks.get_mut(index) {
            Some(value) => {
                value.edit(title);
                Ok(())
            },
            None => Err(Error::Static("Invalid Index".to_string())),
        }
    }
    pub fn mark_task(&mut self, index: usize) -> Result<()> {
        match self.tasks.get_mut(index) {
            Some(value) => {
                value.mark();
                Ok(())
            },
            None => Err(Error::Static("Invalid Index".to_string())),
        }
    }
    pub fn close(&self) -> Result<()> {
        let mut file = File::create("tasks.json")?;
        let json_content = serde_json::to_string(&self.tasks)?;
        file.write_all(json_content.as_bytes())?;

        Ok(())
    }
}
