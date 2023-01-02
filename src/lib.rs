pub mod config;
pub mod result;
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
    pub fn parse(&self) -> std::result::Result<result::Cmd, ParseError> {
        let mut sys_args: Vec<String> = std::env::args().collect();
        for a in sys_args.iter_mut() {
            *a = a.trim().to_owned()
        }
        if sys_args.len() <= 1 {
            println!();
            Err(ParseError::NoArguments)
        }else {
            Ok(self.config_root.parse(&sys_args[1..sys_args.len()]))
        }
    }

    //Check for duplicate names
    fn check(){todo!()}
}

#[derive(Debug, Clone)]
pub enum ParseError {
    NoArguments
}