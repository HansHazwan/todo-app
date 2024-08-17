mod error;
mod prelude;
mod task_manager;
mod displayer;

use crate::prelude::*;
use crate::task_manager::{Title, TaskManager};

fn main() -> Result<()> {
    let mut task_manager = TaskManager::init()?;
    let mut running = true;

    loop {
        let code = displayer::get_home_code();

        match code {
            1 => {
                let title = displayer::get_title();
                let title = Title::new(&title)?;
                task_manager.add_task(title);
            },
            2 => {
                let id = displayer::get_id();
                task_manager.remove_task(id)?;
            },
            3 => {
                let id = displayer::get_id();
                let title = displayer::get_title();
                let title = Title::new(&title)?;
                task_manager.edit_task(id, title)?;
            },
            4 => {
                let id = displayer::get_id();
                task_manager.mark_task(id)?;
            },
            5 => {
                task_manager.print_tasks();
            },
            6 => {
                running = false;
            },
            _ => todo!(),
        }

        if !running {
            break;
        }
    }

    task_manager.close()?;
    Ok(())
}
