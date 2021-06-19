# Command Engine [v3.0.0]
An engine to create your own command-line interpreter. <br>
- No dependencies *(only sync)*
- Commands as user-defined structures
- Asynchronous commands
- Case sensitive commands
- Positional arguments
- Optional non-positional arguments
- Partial "departure" from the conventions

# Example
```rust
struct MyCommand;

impl Command for MyCommand {
    (...)
}

fn main() {
    // Raw instruction in String 
    let raw = String::from("mycommand help");
    
    // Creating the Engine object and adding the command
    let mut engine = Engine::new()
        .add(MyCommand{});
    
    // Executing the instruction
    let out = engine.execute(&raw);
    
    println!("StatusCode: '{}'\n{}", out.result, out.message)
}
```

# Async Example
```rust
struct MyAsyncCommand;

impl Command for MyAsyncCommand {
    (...)
}

#[tokio::main]
async fn main() {
    let raw = String::from("mycommand help");

    let mut engine = AsyncEngine::new()
        .add(MyAsyncCommand{});

    let out = engine.execute(&raw).await;

    println!("StatusCode: '{}'\n{}", out.result, out.message);
}
```
