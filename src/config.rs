//! The config module includes everything to configure the [parser](parser::ArgParser)<br>
//! Includes:
//! - [enum Arg](config::Arg) to defer between Flags, Parameters
//! - [struct Cmd](config::Cmd) to describe sub commands
use std::vec;

/// Enum for configuring arguments
#[derive(Debug, Clone)]
pub enum Arg {
    /// Flags which are false by default and are set to true by being used by the user. <br>
    /// Has its name as argument
    Flag(&'static str),
    /// Parameters which are string inputs
    /// Has its name as argument
    Parameter(&'static str)
}

/// Describes the root and all sub commands. <br>
/// A commands might have arguments ([enum Arg](Arg)) and possible sub commands which are also of type [Cmd](Cmd)
#[derive(Debug)]
pub struct Cmd {
    pub args: &'static[Arg],
    pub sub_cmd: &'static[Cmd]
}

impl Cmd {
    /// Creates an Command without any possible arguments or sub commands
    pub const fn new()->Self {
        Self { args: &[], sub_cmd: &[]}
    }
    /// Creates a Commands having a list of arguments and sub commands
    pub const fn from(args: &'static[Arg], sub_cmd: &'static[Cmd])->Self {
        Self { args, sub_cmd}
    }
    /// Function to parse only this subcommand with the arguments <br>
    /// Meant for use by the [parser](super::parser::ArgParser) internally
    pub fn parse(&self, arguments: &[String])->Result<super::result::Cmd, super::parser::ParseError> {
        use super::parser::ParseError;
        let mut result = super::result::Cmd {
            args: vec![],
            sub_cmd: None,
        };

        let mut skip = 0;        
        for (i, a) in arguments.iter().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if a.starts_with("--") { //Flags
                let name = a.trim_start_matches("--");
                match self.find_arg(name) {
                    Some(a) => match a {
                        Arg::Flag(self_name) => result.args.push(super::result::Arg::Flag(self_name, true)),
                        Arg::Parameter(_) => return Err(ParseError::TypeNameMismatch { name: String::from(name) }),
                    },
                    None => return Err(ParseError::UnknownFlag { name: String::from(name)}),
                }
                
            }else if a.starts_with("-") { // Arguments
                let name = a.trim_start_matches("-");
                match self.find_arg(name) {
                    Some(a) => match a {
                        Arg::Parameter(self_name) => {
                            if i+1 >= arguments.len() {
                                return Err(ParseError::ParameterWithoutValue { name: String::from(name) });
                            }
                            let value = arguments[i+1].clone();
                            skip +=1;
                            result.args.push(super::result::Arg::Parameter(self_name, Some(value)))
                        },
                        Arg::Flag(_) => return Err(ParseError::TypeNameMismatch { name: String::from(name) }),
                    },
                    None => return Err(ParseError::UnknownParameter { name: String::from(name) }),
                }
            } else { //Subcommand
                todo!("Subcommands not implemented")
            }

        }
        Ok(result)        
    }
    //Find an argument by its name in the list of them
    fn find_arg(&self, name: &str)-> Option<&Arg> {
        for self_a in self.args {
            match self_a {
                Arg::Flag(self_name) => {
                    if *self_name == name {
                        return Some(self_a);
                    }
                },
                Arg::Parameter(self_name) => {
                    if *self_name == name {
                        return Some(self_a);
                    }
                },
            }
        }
        None
    }
}