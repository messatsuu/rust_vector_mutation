use crate::io;

mod operation;

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
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        input.trim().to_string()
    }

    pub fn process_operation(&mut self) {
        loop {
            print!("~> ");

            let user_operation: char = match self.read_input().parse() {
                Ok(input) => input,
                Err(_) => {
                    println!("Invalid input. Please enter a single character.");
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
                _ => {
                    println!("operation \"{user_operation}\" not found. Please try again.");
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
                println!("not an integer");
                None
            }
        }
    }

    fn create_element(&mut self) {
        println!("create new element");
        let element_value = self.read_input();
        self.inputs.push(element_value);
        println!("Created sucessfully!");
    }

    fn delete_element(&mut self) {
        if self.inputs.is_empty() {
            println!("No elements to delete");
            return;
        }

        println!("delete an element at given index");
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
            println!("No elements to update");
            return;
        }

        println!("update an element at given index");
        self.list_elements();
        let element_index = self.read_index();

        if let Some(index) = element_index {
            if !self.is_in_bounds(index) {
                return;
            }

            println!("Enter new value:");
            let new_value = self.read_input();
            self.inputs[index] = new_value;
        }
    }

    fn print_help(&self) {
        println!("Should be one of \n{}", self.viable_operations.list_operations());
    }

    fn list_elements(&self) {
        println!("current elements: (\"{}\")", self.inputs.join(", "));
    }

    fn is_in_bounds(&self, index: usize) -> bool {
        let bounds = self.inputs.len() - 1;
        let is_in_bounds = index <= bounds;
        if !is_in_bounds {
            println!("Given index {index} is outside of bounds ({bounds})");
        }

        is_in_bounds
    }
}
