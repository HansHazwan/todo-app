use crate::prelude::*;
use crate::task_manager::Title;
use getset::Getters;
use serde::{Serialize, Deserialize};

#[derive(Debug, Getters, Serialize, Deserialize)]
pub struct Task {
    #[getset(get = "pub")]
    title: Title,

    #[getset(get = "pub")]
    completed: bool,
}

impl Task {
    pub fn new(title: Title) -> Task {
        Task {
            title: title,
            completed: false,
        }
    }
    pub fn edit(&mut self, title: Title) {
        self.title = title;
    }
    pub fn mark(&mut self) {
        self.completed = !self.completed;
    }
}

