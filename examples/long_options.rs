//! # Configuration of long options in the parser
//! The [config/cmd](arg_parse::config::Cmd) accepts as a second argument a slice of [config::LongOption](arg_parse::config::LongOption).<br>
//! A short option contains a [name](str) and the [count](usize) of values which the user has to provide
//! # Parsing input
//! You can parse what the user provided by using [parser.parse()](arg_parse::ArgParser::parse()) or you can use [parser.parse_custom()](arg_parse::ArgParser::parse_custom()) like in this example
//! #Result
//! A nested structure of [result::Cmd](arg_parse::result::Cmd) where each command contains the used arguments including the provided values if configured
//! #Example
//! ```rust,editable
//! fn main() {
//!     use arg_parse::config::{self, LongOption};
//!     let parser = {
//!         let config = config::Cmd::from(&[/*No Long options */ ], &[
//!             LongOption{name: "alpha", value_count: 1}, //Long Option called alpha which needs one value from the user
//!             LongOption{name: "beta", value_count: 0}, //Long option called beta without any values
//!             LongOption{name: "gamma", value_count: 3} //Long option called gamma which need three values from the user
//!         ], &[/*No subcommands */]);
//!         arg_parse::ArgParser::from(config)
//!     };
//! 
//!     //Some example user inputs fitting 
//!     let example_user_args: std::vec::Vec<String> = vec![
//!         "--alpha", "value",
//!         "--beta", 
//!         ].into_iter().map(|s| s.to_string()).collect();
//! 
//!     //Parsing of arguments
//!     let result = match parser.parse_custom(example_user_args) {
//!         Err(msg) => panic!("Err: {:?}", msg),//If error occurs panic for example
//!         Ok(v) => {
//!             v.long_options // On success only return the long options
//!         }
//!     };
//!     println!("Result: {:?}", result);
//! }
//! ```
fn main() {
    use arg_parse::config::{self, LongOption};
    let parser = {
        let config = config::Config::from(&[/*No Long options */ ], &[
            LongOption{name: "alpha", value_count: 1}, //Long Option called alpha which needs one value from the user
            LongOption{name: "beta", value_count: 0}, //Long option called beta without any values
            LongOption{name: "gamma", value_count: 3} //Long option called gamma which need three values from the user
        ], &[/*No subcommands */]);
        arg_parse::ArgParser::from(config)
    };

    //Some example user inputs fitting 
    let example_user_args: std::vec::Vec<String> = vec![
        "--alpha", "value",
        "--beta", 
        ].into_iter().map(|s| s.to_string()).collect();

    //Parsing of arguments
    let result = match parser.parse_custom(example_user_args) {
        Err(msg) => panic!("Err: {:?}", msg),//If error occurs panic for example
        Ok(v) => {
            v.long_options // On success only return the long options
        }
    };
    println!("Result: {:?}", result);
}