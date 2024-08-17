use crate::prelude::*;

pub fn get_home_code() -> usize {
    let mut code = String::new();

    println!("Welcome to Todo App");
    println!("-----------------------");
    println!("1) Add Task");
    println!("2) Remove Task");
    println!("3) Edit Task");
    println!("4) Mark Task");
    println!("5) Print Tasks");
    println!("6) Quit");

    std::io::stdin().read_line(&mut code).expect("Cannot read line");

    code.trim().parse::<usize>().expect("Cannot convert input")
}

pub fn get_title() -> String {
    let mut title = String::new();

    println!("What is your title ? ");
    std::io::stdin().read_line(&mut title).expect("Cannot read line");

    title
}

pub fn get_id() -> usize {
    let mut id = String::new();

    println!("What is the id of the task ? ");
    std::io::stdin().read_line(&mut id).expect("Cannot read line");

    id.trim().parse::<usize>().expect("Cannot convert input")
}

