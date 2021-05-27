use super::*;

#[macro_export]
macro_rules! command_engine_async {
    ( $( $c:expr ),* ) => {
        {
            let mut engine = AsyncEngine::new();
            $(
                engine.add(Box::new($x));
            )*
            engine
        }
    };

}
