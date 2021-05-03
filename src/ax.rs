use super::*;


pub trait Command {
    fn name(&self) -> &str;

    fn on_help(&self) -> &str {
        "Help is not implemend for this command!"
    }

    fn on_execute(&mut self, ins: &Instruction) -> Output;
}
