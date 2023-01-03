use super::*;

/// # The Parser itself 
/// It contains only the "root" command, which is similar to a sub command but always exists. It has its own arguments and sub commands
pub struct ArgParser {
    config_root: config::Cmd,
}

impl ArgParser {
    /// Create a new Parser, giving the "root" command
    /// # Example
    /// ```rust
    /// const ARGS: &'static [config::Arg] = &[config::Arg::Flag("a"), config::Arg::Parameter("b")];
    /// const PARSER_ROOT_CMD: config::Cmd = config::Cmd::from(ARGS, &[]);
    /// static PARSER: ArgParser = ArgParser::from(PARSER_ROOT_CMD);
    /// ```
    pub const fn from(c: config::Cmd)-> Self {
        Self {
            config_root: c
        }
    }
    /// Create an empty parser which doesn't have any contents
    pub const fn new<'f>()-> Self {
        Self {
            config_root: config::Cmd::new()
        }
    }
    /// Function parsing the command line arguments from [std::env::args()](std::env::args()) with the configuration
    /// # Errors
    /// Described with [ParseError](ParseError)
    /// - [NoArguments](ParseError::NoArguments) --> No arguments were provided. Doesn't have to be wrong but leads to no result.
    /// # Panic 
    /// In the future the function should not panic any more!<br>
    /// But for now:
    /// - A Flag was used as an Parameter or vice versa
    /// - A Flag or Parameter which wasn't configured was used
    /// - Subcommands were use (because not implemented yet)

    pub fn parse(&self) -> Result<result::Cmd, ParseError> {
        let mut sys_args: Vec<String> = std::env::args().collect();
        for a in sys_args.iter_mut() {
            *a = a.trim().to_owned()
        }
        if sys_args.len() <= 1 {    //Ignore first argument, because its the application path
            Err(ParseError::NoArguments)
        }else {
            Ok(self.config_root.parse(&sys_args[1..sys_args.len()]))
        }
    }

    /// Does the same as parse but with you own arguments provided as input instead of them from [std::env::args()](std::env::args())
    pub fn parse_custom(&self, mut args: Vec<String>) -> Result<result::Cmd, ParseError> {
        for a in args.iter_mut() {
            *a = a.trim().to_owned()
        }
        if args.len() <= 0 {
            Err(ParseError::NoArguments)
        }else {
            Ok(self.config_root.parse(&args[0..args.len()]))
        }
    }

    //Check for duplicate names
    fn check(){todo!()}
}

#[derive(Debug, Clone)]
pub enum ParseError {
    NoArguments
}