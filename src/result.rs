//! Includes all results of the [parser](parser::ArgParser)<br>
//! Includes:
//! - [enum Arg](result::Arg) includes every kind of Argument and its value
//! - [struct Cmd](result::Cmd) a sub or the "root" command with its arguments and optionally a subcommand


#[derive(Debug, Clone)]
pub struct ShortOption{
    name: char,
    pub values: Vec<String>
}
#[derive(Debug, Clone)]
pub struct LongOption{
    pub name: &'static str,
    pub values: Vec<String>
}

/// Command selected by the user
#[derive(Debug)]
pub struct Cmd {
    pub short_options: Vec<ShortOption>,
    pub long_options: Vec<LongOption>,
    pub sub_cmd: Option<Box<Cmd>>
}

impl Cmd {
    pub fn new()->Self {
        Self { short_options: vec![], long_options: vec![], sub_cmd: None }
    }
}