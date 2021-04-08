use super::*;

const DEFAULT_HELP: &'static str = "Help is not implemend for this command!";


#[deprecated]
pub type CommandFn = fn(&Instruction) -> Output;

pub trait Command {
    fn name(&self) -> &str;

    fn on_help(&self) -> &str {
        DEFAULT_HELP
    }

    fn on_execute(&mut self, ins: &Instruction) -> Output;
}
