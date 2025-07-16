use cli_task_manager::task_manager::cli;
use cli_task_manager::task_manager::cli::*;
use cli_task_manager::task_manager::sync_data_base::*;
use cli_task_manager::task_manager::core_features::*;
use std::io;
use std::io::Write;
use std::process::exit;
use chrono;

fn main() {
    let mut app = App::new();
    app.sync_task_list().unwrap();
    app.get_data_base().refresh_data(app.get_task_list()).unwrap();

    println!("_________________Welcome to TaskManager___________________");
    match menu(&["View tasks", "Exit", "Reset"]) {
        1 => handle_task_actions(&mut app),
        2 => app.exit().unwrap(),
        3 => app.get_data_base().reset_data().unwrap(),
        _ => println!("Invalid option."),
    }
}

fn handle_task_actions(app: &mut App) {
    app.show_data();
    println!("________________________________________________");

    match menu(&["Create task", "Edit task", "Delete task", "Open task"]) {
        1 => {
            let task = TaskBuilder::default();
            app.edit_task(task);
        }
        2 => {
            let id = read_task_id("edit");
            let builder = app.get_task_list_mut().0.remove(id).get_builder();
            app.edit_task(builder);
        }
        3 => {
            let id = read_task_id("delete");
            app.get_task_list_mut().0.remove(id);
            app.get_data_base().refresh_data(app.get_task_list()).unwrap();
        }
        4 => {
            let id = read_task_id("open");
            app.get_task_list_mut().0.remove(id).open_task();
        }
        _ => println!("Invalid option."),
    }
}

fn menu(options: &[&str]) -> usize {
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(0)
}

fn read_task_id(action: &str) -> usize {
    println!("Which task would you like to {}? (number)", action);
    print!("Input: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    id.trim().parse::<usize>().unwrap_or(1) - 1 // convert to 0-indexed
}
