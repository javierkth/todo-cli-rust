use std::fs;
use std::fs::read_to_string;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    if !std::path::Path::new("todos.md").exists() {
        fs::write("todos.md", "").unwrap();
    }
    loop {
        thread::sleep(Duration::from_secs(1));
        std::process::Command::new("clear").status().unwrap();
        println!("=== Todo App===");

        // list tasks
        let content = read_to_string("todos.md").unwrap_or_default();
        println!("{}", content);

        println!("Type <command> task");
        print!("COMMANDS => ");
        print!("add  ");
        print!("delete  ");
        print!("check  ");
        println!("Quit  ");
        print!("> ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let parts: Vec<&str> = input.splitn(2, ' ').collect();
        let command = parts[0];
        let argument = parts.get(1).unwrap_or(&" ");

        match command {
            "add" => {
                let content = read_to_string("todos.md").unwrap_or_default();
                let new_task = format!("- [ ] {}", argument);
                if argument.is_empty() {
                    println!("Task needs at least one letter!");
                } else if content.lines().any(|line| line == new_task) {
                    println!("Task already exists!");
                } else {
                    let mut lines: Vec<&str> = content.lines().collect();
                    lines.push(&new_task);
                    fs::write("todos.md", lines.join("\n") + "\n").unwrap();
                    println!("{} added", argument);
                }
            }
            "delete" => {
                let content = read_to_string("todos.md").unwrap_or_default();
                let uncompleted = format!("- [ ] {}", argument);
                let completed = format!("- [x] {}", argument);

                if !content
                    .lines()
                    .any(|line| line == uncompleted || line == completed)
                {
                    println!("Task not found!");
                } else {
                    let new_content: Vec<&str> = content
                        .lines()
                        .filter(|line| *line != uncompleted && *line != completed)
                        .collect();
                    fs::write("todos.md", new_content.join("\n") + "\n").unwrap();
                    println!("{} deleted", argument);
                }
            }
            "check" => {
                let content = read_to_string("todos.md").unwrap_or_default();
                let new_content = if content.contains(&format!("- [ ] {}", argument)) {
                    content.replace(
                        &format!("- [ ] {}", argument),
                        &format!("- [x] {}", argument),
                    )
                } else if content.contains(&format!("- [x] {}", argument)) {
                    content.replace(
                        &format!("- [x] {}", argument),
                        &format!("- [ ] {}", argument),
                    )
                } else {
                    println!("Task not found!");
                    content
                };
                fs::write("todos.md", new_content).unwrap();
            }
            "quit" => break,
            _ => println!("Unknown command"),
        }
    }
}
