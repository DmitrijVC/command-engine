pub mod ax;

use super::{Output, Instruction};
use std::collections::HashMap;

type CommandObj<'a> = ax::AsyncCommand<'a>;


pub struct AsyncEngine<'a> {
    commands: HashMap<String, CommandObj<'a>>
}

impl<'a> AsyncEngine<'a> {
    pub fn new() -> Self {
        Self {
            commands: HashMap::<String, CommandObj<'a>>::new()
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
                let help = if let Some(help) = command.help {
                    help
                } else {
                    "Help is not implemend for this command!"
                };

                return Output::new_ok(1, Some(String::from(help)));
            }
        }

        command.on_execute(&instruction).await
    }

    pub fn add(mut self, command_struct: CommandObj<'a>) -> Self {
        let name = format!("{}", command_struct.name);
        if let None = self.get_command_mut(&name) {
            self.commands.insert(name, command_struct);
        }

        self
    }

    fn get_command_mut(&mut self, name: &String) -> Option<&mut CommandObj<'a>> {
        match self.commands.get_mut(name) {
            None => None,
            Some(command) => Some(command),
        }
    }
}
