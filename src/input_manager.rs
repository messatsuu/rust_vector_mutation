use crate::io;
use std::{io::Write, process::Command};
use colored::Colorize;

mod operation;

enum LogLevel {
    Success,
    Info,
    Error
}

impl LogLevel {
    fn to_color(&self, message: &str) -> colored::ColoredString {
        match self {
            LogLevel::Success => message.bold().green(),
            LogLevel::Info => message.bold().blue(),
            LogLevel::Error => message.bold().red(),
        }
    }

    fn to_string(&self) -> String {
        match self {
            LogLevel::Success => "Success",
            LogLevel::Info => "Info",
            LogLevel::Error => "Error",
        }.to_string()
    }
}

macro_rules! prompt_output {
    ($str: expr, $level: expr $(, $args:tt)*) => {
        let formatted_message = format!($str, $($args)*);
        println!("[{}] {}", $level.to_string(), $level.to_color(&formatted_message));
    };
}

pub struct InputManager {
    inputs: Vec<String>,
    viable_operations: operation::Operations,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            inputs: Vec::new(),
            viable_operations: operation::Operations::new(),
        }
    }

    fn read_input(&self) -> String {
        print!("> ");
        // This makes sure that the prompt is displayed before the user input (by flushing the buffer)
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        input.trim().to_string()
    }

    pub fn process_operation(&mut self) {
        loop {
            let user_operation: char = match self.read_input().parse() {
                Ok(input) => input,
                Err(_) => {
                    prompt_output!("Invalid input. Please enter a single character.", LogLevel::Error);
                    continue;
                }
            };

            match user_operation {
                // TODO: put operations into `Operation` struct
                'c' => self.create_element(),
                'd' => self.delete_element(),
                'u' => self.update_element(),
                'l' => self.list_elements(),
                'h' => self.print_help(),
                'x' => self.clear_screen(),
                _ => {
                    prompt_output!("operation \"{user_operation}\" not found. Type 'h' to see available options.", LogLevel::Error);
                    continue;
                }
            }
        }
    }

    fn read_index(&self) -> Option<usize> {
        let index_str = self.read_input();
        match index_str.parse::<usize>() {
            Ok(index) => Some(index),
            Err(_) => {
                prompt_output!("not an integer", LogLevel::Error);
                None
            }
        }
    }

    fn create_element(&mut self) {
        prompt_output!("create new element", LogLevel::Info);
        let element_value = self.read_input();
        self.inputs.push(element_value);
        prompt_output!("Created sucessfully!", LogLevel::Success);
    }

    fn delete_element(&mut self) {
        if self.inputs.is_empty() {
            prompt_output!("No elements to delete", LogLevel::Info);
            println!("No elements to delete");
            return;
        }

        prompt_output!("delete an element at given index", LogLevel::Info);
        self.list_elements();

        let element_index = self.read_index();

        if let Some(index) = element_index {
            if !self.is_in_bounds(index) {
                return;
            }

            self.inputs.remove(index);
        }
    }

    fn update_element(&mut self) {
        if self.inputs.is_empty() {
            prompt_output!("No elements to update", LogLevel::Info);
            return;
        }

        prompt_output!("update an element at given index", LogLevel::Info);
        self.list_elements();
        let element_index = self.read_index();

        if let Some(index) = element_index {
            if !self.is_in_bounds(index) {
                return;
            }

            prompt_output!("Enter new value:", LogLevel::Info);
            let new_value = self.read_input();
            self.inputs[index] = new_value;
            prompt_output!("Value Updated!", LogLevel::Success);
        }
    }

    fn print_help(&self) {
        let operations = self.viable_operations.list_operations();
        prompt_output!("Should be one of \n{operations}", LogLevel::Info);
    }

    fn clear_screen(&self) {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }

    fn list_elements(&self) {
        if self.inputs.is_empty() {
            prompt_output!("The list is currently empty.", LogLevel::Info);
            return;
        }
        let inputs = self.inputs.join(", ");
        prompt_output!("current elements:\n\"{inputs}\"", LogLevel::Info);
    }

    fn is_in_bounds(&self, index: usize) -> bool {
        let bounds = self.inputs.len() - 1;
        let is_in_bounds = index <= bounds;
        if !is_in_bounds {
            prompt_output!("Given index {index} is outside of bounds ({bounds})", LogLevel::Error);
        }

        is_in_bounds
    }
}
