use super::*;


/// Trait allowing the struct to be used as a Command in the synchronous Engine
pub trait Command {
    /// Returns name of the Command by which it will be detected
    fn name(&self) -> &str;

    /// Returns help message when `<CommandName> help` was called
    fn on_help(&self) -> &str {
        "Help is not implemented for this command!"
    }

    /// Logic that executes when the Instruction had the Command's name
    ///
    /// Returns an Output indicating in the success or failure of the Command execution
    fn on_execute(&mut self, ins: &Instruction) -> Output;
}
