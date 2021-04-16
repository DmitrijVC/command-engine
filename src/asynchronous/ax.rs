use crate::{Instruction, Output};
use futures::future::BoxFuture;


pub type AsyncOnExecute<'b> = Box<dyn for<'a> FnMut(&Instruction) -> BoxFuture<'b, Output>>;

pub struct AsyncCommand<'b> {
    pub name: &'b str,
    pub help: Option<&'b str>,
    pub execute_fun: AsyncOnExecute<'b>,
}

impl<'b> AsyncCommand<'b> {
    pub fn new(name: &'b str, help: Option<&'b str>, execute_fun: AsyncOnExecute<'b>) -> Self {
        Self {
            name,
            help,
            execute_fun
        }
    }

    pub async fn on_execute(&mut self, ins: &Instruction) -> Output {
        (self.execute_fun)(ins).await
    }
}
