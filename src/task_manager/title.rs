use crate::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Title(String);

impl Default for Title {
    fn default() -> Title {
        Title("Default task for testing.".to_string())
    }
}

impl Title {
    pub fn new(title: &str) -> Result<Title> {
        if title.len() > 30 || title.is_empty() {
            return Err(Error::Static("Task title should not more than 30 words or empty".to_string()));
        }

        Ok(Title(title.to_owned()))
    }
    pub fn value(&self) -> String {
        self.0.clone()
    }
}

