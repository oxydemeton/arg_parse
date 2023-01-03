//! The config module includes everything to configure the [parser](parser::ArgParser)<br>
//! Includes:
//! - [enum Arg](config::Arg) to defer between Flags, Parameters
//! - [struct Cmd](config::Cmd) to describe sub commands
use std::vec;


#[derive(Debug, Clone)]
pub struct ShortOption{
    pub name: char,
    pub value_count: usize
}
#[derive(Debug, Clone)]
pub struct LongOption{
    pub name: &'static str,
    pub value_count: usize
}

/// Describes the root and all sub commands. <br>
/// A commands might have arguments ([enum Arg](Arg)) and possible sub commands which are also of type [Cmd](Cmd)
#[derive(Debug)]
pub struct Cmd {
    pub short_options: &'static [ShortOption],
    pub long_options: &'static [LongOption],
    pub sub_cmd: &'static[Cmd]
}

impl Cmd {
    /// Creates an Command without any possible arguments or sub commands
    pub const fn new()->Self {
        Self { long_options: &[], short_options: &[], sub_cmd: &[]}
    }
    /// Creates a Commands having a list of arguments and sub commands
    pub const fn from(short_options: &'static [ShortOption], long_options: &'static [LongOption], sub_cmd: &'static[Cmd])->Self {
        Self { short_options, long_options,  sub_cmd}
    }
    /// Function to parse only this subcommand with the arguments <br>
    /// Meant for use by the [parser](super::parser::ArgParser) internally
    pub fn parse(&self, arguments: &[String])->Result<super::result::Cmd, super::parser::ParseError> {
        use super::parser::ParseError;
        use super::result;
        let mut result = super::result::Cmd::new();

        let mut skip = 0;        
        for (i, a) in arguments.iter().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            //LongOption Parsing
            if a.starts_with("--") { 
                let option_option = self.find_long_option(a);
                match option_option {
                    Err(name) => return Err(ParseError::UnknownLongOption { name: String::from(name)}),
                    Ok(option) => {
                        if option.value_count == 0 {
                            result.long_options.push(result::LongOption{name: option.name, values: vec![String::new()]})
                        }else {
                            let mut option_result = result::LongOption{name: option.name, values: vec![]};
                            if option.value_count >= arguments.len() {
                                return Err(ParseError::ParameterWithoutEnoughValues { name: String::from(option.name) })
                            }
                            skip = option.value_count;
                            for i_value in i+1..i+option.value_count+1 {
                                option_result.values.push(arguments[i_value].clone());
                            }
                            result.long_options.push(option_result);
                        }
                    }
                }

            }else if a.starts_with("-") { // Short Options
                todo!("Short options are not implemented")
            } else { //Subcommand
                todo!("Subcommands not implemented")
            }

        }
        Ok(result)        
    }
    //Find an argument by its name in the list of them
    fn find_short_option(&self, name: &char)-> Option<&ShortOption> {
        for o in self.short_options {
            if o.name == *name {
                return Some(o)
            }
        }
        None
    }
    fn find_long_option<'a>(&self, name_input: &'a str)-> Result<&LongOption, &'a str> {
        let name = name_input.trim_start_matches("--");
        for o in self.long_options {
            if o.name == name {
                return Ok(o)
            }
        }
        Err(name)
    }
}