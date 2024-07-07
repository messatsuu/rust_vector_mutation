pub struct Operations {
    pub operations: Vec<Operation>,
}

impl Operations {
    pub fn new() -> Self {
        let create = Operation::new('c', "creates a new element");
        let update = Operation::new('u', "updates an element");
        let delete = Operation::new('d', "deletes an element");
        let list = Operation::new('l', "list all elements");
        Operations {
            operations: vec![create, update, delete, list],
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
