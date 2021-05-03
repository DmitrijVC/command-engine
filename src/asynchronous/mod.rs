pub mod ax;

pub use async_trait::async_trait;
pub use ax::*;
use super::{Output, Instruction};
use std::collections::HashMap;


pub struct AsyncEngine {
    commands: HashMap<String, Box<dyn AsyncCommand>>
}

impl AsyncEngine {
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, Box<dyn AsyncCommand>>::new()
        }
    }

    pub async fn execute(&mut self, raw_instruction: &String) -> Output {
        let instruction = match Instruction::new(raw_instruction) {
            Ok(instruction) => instruction,
            Err(output) => return output,
        };

        let command = match self.get_command_mut(&instruction.value) {
            None => return Output::new_error(1, Some(String::from("Invalid command!"))),
            Some(command) => command,
        };

        if let Some(arg0) = instruction.args.get(0) {
            if arg0.eq("help") {
                return Output::new_ok(1, Some(String::from(command.on_help())));
            }
        }

        command.on_execute(&instruction).await
    }

    pub fn add(mut self, command_struct: Box<dyn AsyncCommand>) -> Self {
        let name = format!("{}", command_struct.name());
        if let None = self.get_command(&name) {
            self.commands.insert(command_struct.name().to_string(), command_struct);
        }

        self
    }

    fn get_command(&self, name: &String) -> Option<&Box<dyn AsyncCommand>> {
        match self.commands.get(name) {
            None => None,
            Some(command) => Some(command),
        }
    }

    fn get_command_mut(&mut self, name: &String) -> Option<&mut Box<dyn AsyncCommand>> {
        match self.commands.get_mut(name) {
            None => None,
            Some(command) => Some(command),
        }
    }
}
