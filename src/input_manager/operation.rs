use colored::Colorize;

pub struct Operations {
    pub operations: Vec<Operation>,
}

impl Operations {
    pub fn new() -> Self {
        let create = Operation::new('c', "create a new element");
        let update = Operation::new('u', "update an element");
        let delete = Operation::new('d', "delete an element");
        let list = Operation::new('l', "list all elements");
        let help = Operation::new('h', "list help");
        let clear = Operation::new('x', "clear screen");
        Operations {
            operations: vec![create, update, delete, list, help, clear],
        }
    }

    pub fn list_operations(&self) -> String {
        self.operations
            .iter()
            .map(|operation| format!(
                "({}) {}",
                operation.operation_char, operation.description
            ))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub struct Operation {
    pub operation_char: char,
    pub description: String,
}

impl Operation {
    fn new(operation: char, description: &str) -> Self {
        Operation {
            operation_char: operation,
            description: description.to_string(),
        }
    }
}
