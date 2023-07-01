use polymenu_common::item;
use std::error::Error;
use std::io;
use std::process::Command;

const QUERY_VAR_NAME: &str = "$QUERY";

pub struct Callback {
    program: String,
    args: Vec<String>,
}

impl Callback {
    pub fn new(cli_args: Vec<String>) -> Self {
        let program = cli_args
            .get(0)
            .unwrap_or_else(|| unreachable!("Clap should force at least one argument for callback"))
            .to_string();
        let args = cli_args.iter().skip(1).map(String::from).collect();
        Self { program, args }
    }

    pub fn call(&mut self, query: &str) -> Result<Vec<item::Item>, Box<dyn Error>> {
        let output = Command::new(&self.program)
            .args(
                self.args
                    .iter()
                    .map(|a| if a == QUERY_VAR_NAME { query } else { a }),
            )
            .output()?;
        item::parse_items(io::Cursor::new(output.stdout))
    }
}
