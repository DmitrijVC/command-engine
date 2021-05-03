# Command Engine
A simple engine to create your own command-line interpreter. <br>
Still needs a little tweaking and rewriting some functions.
- No dependencies *(only sync)*
- Commands as user-defined structures
- Asynchronous commands
- Case sensitive commands
- Positional arguments
- Optional non-positional arguments
- Partial "departure" from the conventions

# [Important Note](https://github.com/DmitrijVC/command-engine/blob/master/src/lib.rs#L2)
```
This library is a part of a private project, and is specially designed for it.
I do not guarantee the full functionality of this library in your project.
Any pull requests colliding with the core program will be denied.
```

# Example
```rust
// Create a struct of the command
struct CommandAddition {
    name: String
}

// Implement basic behaviour
impl CommandAddition {
    pub fn init() -> Self {
        Self {
            name: String::from("add")
        }
    }
}

// Implement Command trait
impl Command for CommandAddition {

    // Returns name of the command
    fn name(&self) -> &str {
        &*(self.name)
    }

    // String returned when help was called
    fn on_help(&self) -> &str {
        "Adds two positive numbers.\nadd <positive number> <positive number>\n\n"
    }

    // Logic executed when command was called
    fn on_execute(&mut self, ins: &Instruction) -> Output {
        if ins.args.len() != 2 {
            return Output::new_error(6, Some(String::from("Invalid number of arguments!")))
        }

        let x = if let Ok(x) = u32::from_str( ins.args.get(0).unwrap() ) {
            x
        } else {
            return Output::new_error(7, Some(String::from("Argument 1 is not a positive number!")))
        };

        let y = if let Ok(y) = u32::from_str( ins.args.get(1).unwrap() ) {
            y
        } else {
            return Output::new_error(7, Some(String::from("Argument 2 is not a positive number!")))
        };

        Output::new_ok(6, Some(format!("{}", x+y)))
    }
}

fn main() {
    // Raw instruction in String 
    let raw = String::from("test help");
    
    // Creating the Engine object
    let mut engine = Engine::new()
        .add(Box::new(CommandAddition::init()));
    
    // Executing the instruction
    let out = engine.execute(&raw);
    
    println!("StatusCode: '{}'\n{}", out.result, out.message)
}
```

# Async Example
```rust
use reqwest;
use command_engine::asynchronous::{AsyncCommand, AsyncEngine, async_trait};
use command_engine::{Output, Instruction};


struct CommandWanip;

#[async_trait]
impl AsyncCommand for CommandWanip {
    fn name(&self) -> &str {
        "wanip"
    }

    async fn on_execute(&mut self, _: &Instruction) -> Output {
        return if let Ok(response) = reqwest::get("https://api.ipify.org/?format=txt").await {
            Output::new_ok(6, Some(response.text().await.unwrap()))
        } else {
            Output::new_error(6, Some(String::from("Couldn't get the result from ipify.")))
        }
    }
}

#[tokio::main]
async fn main() {
    let raw = String::from("wanip");

    let mut engine = AsyncEngine::new()
        .add(Box::new(CommandWanip{}));

    let out = engine.execute(&raw).await;
    println!("StatusCode: '{}'\n{}", out.result, out.message);
}
```
