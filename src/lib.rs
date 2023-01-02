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
    pub fn parse(&self) -> result::Cmd {
        let sys_args: Vec<String> = std::env::args().collect();    
        self.config_root.parse(&sys_args)
    }

    //Check for duplicate names
    fn check(){todo!()}
}