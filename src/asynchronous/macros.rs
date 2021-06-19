use super::*;


/// Creates an asynchronous Engine object with specific Commands
///
/// # Arguments
///
/// * `command_struct` - Struct implementing the Command trait (can repeat)
///
/// # Example
/// ```
/// #[macro_use] extern crate command_engine;
///
/// let mut engine = command_engine_async!(MyCommand{}, MyCommand2{});
/// ```
#[macro_export]
macro_rules! command_engine_async {
    ( $( $c:expr ),* ) => {
        {
            let mut engine = AsyncEngine::new();
            $(
                engine.add_separated($c);
            )*
            engine
        }
    };

}
