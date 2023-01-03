//! The config module includes everything to configure the [parser](parser::ArgParser)<br>
//! Includes:
//! - [ShortOptions](config::ShortOption)
//! - [LongOptions](config::LongOption)
//! - [Commands](config::Cmd) a sub or the "root" command with its arguments and optionally a subcommand
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
/// A commands might have [short](ShortOption) and [long](LongOption) options and possible sub commands which are also of type [Cmd](Cmd)
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
                let options_option = self.find_short_options(a);
                match options_option {
                    Err(name) => return Err(ParseError::UnknownShortOption { name: String::from(name) }),
                    Ok(options) => {
                        for (i_option, option) in options.iter().enumerate() {
                            let mut option_result = result::ShortOption{name: option.name, values: vec![]};
                            if i_option == options.len()-1 && option.value_count > 0 {
                                skip = option.value_count;
                                for i_value in i+1..i+option.value_count+1 {
                                    option_result.values.push(arguments[i_value].clone());
                                }
                            }else if option.value_count > 0 {
                                return Err(ParseError::ShortOptionMissingValue { name: String::from(option.name) })
                            }else {
                                option_result.values.push(String::new());
                            }
                            result.short_options.push(option_result);
                        }
                    }
                }
            } else { //Subcommand
                todo!("Subcommands not implemented")
            }

        }
        Ok(result)        
    }
    //Find an argument by its name in the list of them
    fn find_short_options(&self, input_names: &str)-> Result<Vec<&ShortOption>, char> {
        let names = input_names.trim_start_matches("-");
        let mut result = vec![];
        for c in names.chars() {
            let len_before = result.len();
            for o in self.short_options {
                if o.name == c {
                    result.push(o)
                }
            }
            if result.len() == len_before {
                return Err(c)
            }
        }
        Ok(result)
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