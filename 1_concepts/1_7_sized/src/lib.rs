use std::process::Command;
use step_1_6::*;
trait CommandHandler<C: Command> {
    type Context: ?Sized;
    type Result;

    fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
}

impl CommandHandler<CreateUser> for User{
    type Context = dyn UserRepository;
    
    type Result = Result<(), UserError>;
    
    fn handle_command(&self, cmd: &CreateUser, ctx: &Self::Context) -> Self::Result {
         // Here we operate with the `UserRepository`
        // via its trait object `&dyn UserRepository`
        todo!()
    }
}