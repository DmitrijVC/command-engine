use crate::{Instruction, Output};
use async_trait;


#[async_trait]
pub trait AsyncCommand: Send + Sync {
    fn name(&self) -> &str;

    fn on_help(&self) -> &str {
        "Help is not implemend for this command!"
    }

    async fn on_execute(&mut self, ins: &Instruction) -> Output;
}
