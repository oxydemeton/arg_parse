use super::*;
pub struct ArgParser {
    config_root: config::Cmd,
}

impl ArgParser {
    pub const fn from(c: config::Cmd)-> Self{
        Self {
            config_root: c
        }
    }
    pub const fn new<'f>()-> Self {
        Self {
            config_root: config::Cmd::new()
        }
    }
    pub fn parse(&self) -> Result<result::Cmd, ParseError> {
        let mut sys_args: Vec<String> = std::env::args().collect();
        for a in sys_args.iter_mut() {
            *a = a.trim().to_owned()
        }
        if sys_args.len() <= 1 {
            Err(ParseError::NoArguments)
        }else {
            Ok(self.config_root.parse(&sys_args[1..sys_args.len()]))
        }
    }

    pub fn parse_custom(&self, mut args: Vec<String>) -> Result<result::Cmd, ParseError> {
        for a in args.iter_mut() {
            *a = a.trim().to_owned()
        }
        if args.len() <= 1 {
            Err(ParseError::NoArguments)
        }else {
            Ok(self.config_root.parse(&args[1..args.len()]))
        }
    }

    //Check for duplicate names
    fn check(){todo!()}
}

#[derive(Debug, Clone)]
pub enum ParseError {
    NoArguments
}