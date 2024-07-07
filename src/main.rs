mod input_manager;

use std::io;
use input_manager::InputManager;


fn main() {
    let mut input_manager = InputManager::new();
    input_manager.process_operation();
}
