use crate::{Instruction, Output};
use async_trait;


/// Trait allowing the struct to be used as a Command in the asynchronous Engine
#[async_trait]
pub trait AsyncCommand: Send + Sync {
    /// Returns name of the Command by which it will be detected
    fn name(&self) -> &str;

    /// Returns help message when `<CommandName> help` was called
    fn on_help(&self) -> &str {
        "Help is not implemend for this command!"
    }

    /// Logic that executes when the Instruction had the Command's name
    ///
    /// Returns an Output indicating in the success or failure of the Command execution
    async fn on_execute(&mut self, ins: &Instruction) -> Output;
}
