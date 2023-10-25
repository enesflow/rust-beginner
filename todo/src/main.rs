use std::fs::File;
use std::io::prelude::*;

fn initialize(filename: &str) {
    // check if it already exists
    let f = File::open(filename);
    match f {
        Ok(_) => {}
        Err(_) => {
            // create the file
            let mut file = File::create(filename).expect("Could not create file");
            match file.write_all(b"Your first task\n") {
                Ok(_) => println!("File created"),
                Err(_) => println!("Could not write to file"),
            }
        }
    }
}

fn get_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Could not open file");
    let lines = std::io::BufReader::new(file).lines();
    let mut result = Vec::new();
    for line in lines {
        result.push(line.unwrap());
    }
    result
}

fn toggle_line(lines: Vec<String>, line_number: usize, to: bool) -> Vec<String> {
    let mut result = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        if index == line_number - 1 {
            if line.starts_with("N") && to {
                result.push(line.replace("N", "Y"));
            } else if line.starts_with("Y") && !to {
                result.push(line.replace("Y", "N"));
            } else {
                result.push(line.to_string());
            }
        } else {
            result.push(line.to_string());
        }
    }
    result
}

fn verify_line_number(lines: Vec<String>, line_number: usize) -> bool {
    if line_number <= 0 {
        println!("Line number must be greater than 0");
        return false;
    }
    if line_number > lines.len() {
        println!("Line number is too big");
        return false;
    }
    true
}

fn write_clear(filename: &str, content: String) {
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .open(filename)
        .expect("Could not open file");
    match file.set_len(0) {
        Ok(_) => {}
        Err(_) => {
            println!("Could not clear file");
            return;
        }
    }
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Written to file"),
        Err(_) => println!("Could not write to file"),
    }
}

fn main() {
    let filename = "todo.txt";
    initialize(filename);
    let argument = std::env::args().nth(1).expect("Please specify an argument");
    match argument.as_str() {
        "add" => {
            let task = std::env::args().nth(2).expect("Please specify a task");
            // add a new line to the end
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(filename)
                .expect("Could not open file");
            match file.write_all(format!("N {}\n", task).as_bytes()) {
                Ok(_) => println!("Task added"),
                Err(_) => println!("Could not write to file"),
            }
        }
        "remove" => {
            // expect the line number
            let line_number = std::env::args()
                .nth(2)
                .expect("Please specify a line number")
                .parse::<usize>()
                .expect("Line number must be an integer");
            // make sure that the line number is valid and doesn't overflow
            let lines = get_lines(filename);
            if !verify_line_number(lines.clone(), line_number) {
                return;
            }
            // remove the line
            let mut result = String::new();
            for (index, line) in lines.iter().enumerate() {
                if index != line_number - 1 {
                    result.push_str(line);
                    result.push_str("\n");
                }
            }
            write_clear(filename, result);
        }
        "list" => {
            println!("Here are your tasks:");
            // print with the line numbers
            let lines = get_lines(filename);
            if lines.len() == 0 {
                println!("No tasks");
                return;
            }
            for (index, line) in lines.iter().enumerate() {
                println!("{}: {}", index + 1, line);
            }
        }
        "done" => {
            // expect the line number
            let line_number = std::env::args()
                .nth(2)
                .expect("Please specify a line number")
                .parse::<usize>()
                .expect("Line number must be an integer");
            // make sure that the line number is valid and doesn't overflow
            let lines = get_lines(filename);
            if !verify_line_number(lines.clone(), line_number) {
                return;
            }
            // toggle the line
            let result = toggle_line(lines, line_number, true);
            write_clear(filename, result.join("\n"));
        }
        "undone" => {
            // expect the line number
            let line_number = std::env::args()
                .nth(2)
                .expect("Please specify a line number")
                .parse::<usize>()
                .expect("Line number must be an integer");
            // make sure that the line number is valid and doesn't overflow
            let lines = get_lines(filename);
            if !verify_line_number(lines.clone(), line_number) {
                return;
            }
            // toggle the line
            let result = toggle_line(lines, line_number, false);
            write_clear(filename, result.join("\n"));
        }
        _ => println!("Invalid argument. Expected: add, remove, list, done, undone"),
    }
}
