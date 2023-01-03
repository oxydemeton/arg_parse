//! Includes all results of the [parser](parser::ArgParser)<br>
//! Includes:
//! - [ShortOptions](result::ShortOption)
//! - [LongOptions](result::LongOption)
//! - [Commands](result::Cmd) a sub or the "root" command with its arguments and optionally a subcommand

/// Description of a used short option <br>
/// ## Usage in command line:
/// - `-n` for a short option called `n` without any parameter
/// - `-n value` for a option called `n` with one value
/// - `-ab` for a short option called `a` and one called `b`
/// - `-ab value-one value-two` for a short option called `a` without parameters and one called `b` with two
#[derive(Debug, Clone)]
pub struct ShortOption{
    pub name: char,
    pub values: Vec<String>
}

/// Description of a used long option <br>
/// ## Usage in command line
/// - `--name` for a long argument called `name`
/// - `--name value` for a long argument called `name` with one parameter
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