use super::*;
use std::fmt::{Formatter, Display};


// ToDo: Rewrite needed
fn parser(raw: &str) -> Vec<String> {
    let mut commands = Vec::<String>::new();

    let mut fake_split = false;
    let mut spaceable = false;
    let mut tmp = String::new();
    for ch in raw.chars().into_iter() {
        if ch == '"' && !fake_split {
            spaceable = !spaceable;

            if tmp.is_empty() {
                continue;
            }
            commands.push(tmp.clone());
            tmp.clear();

            continue;
        }

        if fake_split {
            fake_split = false
        } else if ch == '\\' {
            fake_split = true;
            continue;
        }

        if spaceable {
            tmp.push(ch);
        } else {
            if ch == ' ' {

                if tmp.is_empty() {
                    continue;
                }
                commands.push(tmp.clone());
                tmp.clear();

                continue;
            } else {
                tmp.push(ch);
            }
        }
    }

    if !tmp.is_empty() {
        commands.push(tmp);
    }

    commands
}


#[derive(Default, Debug)]
pub struct Instruction {
    pub value: String,
    pub args: Vec<String>,
    pub oargs: HashMap<String, Option<String>>,
}

impl Instruction {
    // ToDo: Rewrite needed
    pub fn new(raw: &str) -> StdResult<Self, Output> {
        let commands = parser(raw);

        if commands.is_empty() {
            return Err(
                Output::new_error(0, Some(String::from("Invalid instruction!")))
            )
        }

        let value = commands.get(0).unwrap().clone();
        let mut args = Vec::<String>::new();
        let mut oargs = HashMap::<String, Option<String>>::new();

        let mut tmp_key = String::new();
        let mut is_flag = false;
        for x in &commands[1..] {
            if x.starts_with("-") {
                if is_flag {
                    oargs.insert(tmp_key.clone(), None);
                } else {
                    is_flag = true;
                }

                tmp_key.clear();
                tmp_key = x.clone();
                continue;
            }

            if is_flag {
                is_flag = false;
                oargs.insert(tmp_key.clone(), Some(x.clone()));
                tmp_key.clear();
                continue;
            }

            args.push(x.clone());
        }

        if !tmp_key.is_empty() {
            oargs.insert(tmp_key.clone(), None);
        }

        Ok( Self {
            value,
            args,
            oargs,
        } )
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Command: [\n\t{}\n]\nArgs: [\n\t{:?}\n]\nOargs: [\n\t{:?}\n]",
            &self.value,
            &self.args,
            &self.oargs,
        )
    }
}