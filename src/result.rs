//! Includes all results of the [parser](parser::ArgParser)
//! Includes:
//! - [enum Arg](result::Arg) includes every kind of Argument and its value
//! - [struct Cmd](result::Cmd) a sub or the "root" command with its arguments and optionally a subcommand

/// includes every kind of Argument and its value
#[derive(Debug, Clone)]
pub enum Arg {
    /// A Flag selected by the user which contains its name and if it was selected
    Flag(&'static str, bool),
    /// A Parameter selected by the user and it's value as a String
    Parameter(&'static str, Option<String>)
}

/// Command selected by the user
#[derive(Debug)]
pub struct Cmd {
    /// All given Arguments to this specific command
    pub args: Vec<Arg>,
    /// If provided the given subcommand
    pub sub_cmd: Option<Box<Cmd>>
}