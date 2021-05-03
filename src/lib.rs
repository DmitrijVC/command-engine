/*
    This library is a part of a private project, and is specially designed for it.
    I do not guarantee the full functionality of this library in your project.
    Any pull requests colliding with the core program will be denied.
*/

#[cfg(feature = "async")]
pub mod asynchronous;

#[cfg(feature = "async")]
#[macro_use] extern crate async_trait;

mod instruction;
mod output;
mod ax;

pub use instruction::*;
pub use output::Output;
pub use output::Result;
pub use ax::*;
use std::collections::HashMap;
use std::result::Result as StdResult;
use std::fmt::Result as FmtResult;


pub struct Engine {
    commands: HashMap<String, Box<dyn Command>>
}

impl Engine {
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, Box<dyn Command>>::new()
        }
    }

    pub fn execute(&mut self, raw_instruction: &String) -> Output {
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

        command.on_execute(&instruction)
    }

    pub fn add(mut self, command_struct: Box<dyn Command>) -> Self {
        let name = format!("{}", command_struct.name());
        if let None = self.get_command(&name) {
            self.commands.insert(command_struct.name().to_string(), command_struct);
        }

        self
    }

    fn get_command(&self, name: &String) -> Option<&Box<dyn Command>> {
        match self.commands.get(name) {
            None => None,
            Some(command) => Some(command),
        }
    }

    fn get_command_mut(&mut self, name: &String) -> Option<&mut Box<dyn Command>> {
        match self.commands.get_mut(name) {
            None => None,
            Some(command) => Some(command),
        }
    }
}
