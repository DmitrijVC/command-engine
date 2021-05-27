use super::*;

#[macro_export]
macro_rules! command_engine {
    ( $( $c:expr ),* ) => {
        {
            let mut engine = Engine::new();
            $(
                engine.add(Box::new($x));
            )*
            engine
        }
    };

}
