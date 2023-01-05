use super::*;

/// # The Parser itself 
/// It contains only the "root" command, which is similar to a sub command but always exists. It has its own arguments and sub commands
pub struct ArgParser {
    config_root: config::Config,
}

impl ArgParser {
    /// Create a new Parser, giving the "root" command
    /// # Example
    /// ```rust
    /// use arg_parse::ArgParser;
    /// use arg_parse::config;
    /// const LONG_OPTIONS: &'static [config::LongOption] = &[
    /// config::LongOption{name: "hello", value_count: 0}
    ///     ];
    /// const SHORT_OPTIONS: &'static [config::ShortOption] = &[
    ///     config::ShortOption{name:'b', value_count: 2},
    ///     config::ShortOption{name:'a', value_count: 0}
    ///     ];
    /// const PARSER_ROOT_CMD: config::Config = config::Config::from(SHORT_OPTIONS, LONG_OPTIONS, &[]);
    /// ```
    pub const fn from(c: config::Config)-> Self {
        Self {
            config_root: c
        }
    }
    /// Create an empty parser which doesn't have any contents
    pub const fn new<'f>()-> Self {
        Self {
            config_root: config::Config::new()
        }
    }
    /// Function parsing the command line arguments from [std::env::args()](std::env::args()) with the configuration
    /// # Errors
    /// Described in [ParseError](ParseError)

    pub fn parse(&self) -> Result<result::Root, ParseError> {
        let mut sys_args: Vec<String> = std::env::args().collect();
        for a in sys_args.iter_mut() {
            *a = a.trim().to_owned()
        }
        if sys_args.len() <= 1 {    //Ignore first argument, because its the application path
            Err(ParseError::NoArguments)
        }else {
            self.config_root.parse(&sys_args[1..sys_args.len()])
        }
    }

    /// Does the same as parse but with you own arguments provided as input instead of them from [std::env::args()](std::env::args())
    pub fn parse_custom(&self, mut args: Vec<String>) -> Result<result::Root, ParseError> {
        for a in args.iter_mut() {
            *a = a.trim().to_owned()
        }
        if args.len() <= 0 {
            Err(ParseError::NoArguments)
        }else {
            self.config_root.parse(&args[0..args.len()])
        }
    }

    //Check for duplicate names
    fn check(){todo!()}
}

#[derive(Debug, Clone)]
pub enum ParseError {
    /// No arguments were provided. Doesn't have to be wrong but leads to no result.
    NoArguments,
    /// The user provided a `short option` which isn't defined
    UnknownShortOption{ name: String },
    /// The user provided a `long option` which isn't defined
    UnknownLongOption{ name: String },
    /// The user provided a `non option` which isn't defined
    UnknownNonOption{ name: String },
    /// The user provided a parameter but did not gave a value
    ParameterWithoutEnoughValues{ name: String },
    /// A Short Option which expects values was used inside a combination of multiple once
    ShortOptionMissingValue{ name: String },
}