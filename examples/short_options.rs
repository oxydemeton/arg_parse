//! # Configuration of short options in the parser
//! The [config](arg_parse::config::Config) accepts an slice of [config::ShortOption](arg_parse::config::ShortOption).<br>
//! A short option contains a [name](char) and the [count](usize) of values which the user has to provide
//! # Parsing input
//! You can parse what the user provided by using [parser.parse()](arg_parse::ArgParser::parse()) or you can use [parser.parse_custom()](arg_parse::ArgParser::parse_custom()) like in this example
//! #Result
//! A nested structure of [result::Root](arg_parse::result::Root) where each command contains the used arguments including the provided values if configured
//! #Example
//! ```rust,editable
//! fn main() {
//!     use arg_parse::config::{self, ShortOption};
//!     let parser = {
//!         let config = config::Config::from(&[
//!             ShortOption{name: 'a', value_count: 0}, // Short Option called a which doesn't accept any arguments
//!             ShortOption{name: 'b', value_count: 1}, // Short Option called b which accepts one argument
//!             ShortOption{name: 'c', value_count: 3} // Short Option called c which accepts two arguments
//!         ], &[/*No long options*/], &[/*No non options */]);
//!         arg_parse::ArgParser::from(config)
//!     };
//! 
//!     //Some example user inputs fitting 
//!     let example_user_args: std::vec::Vec<String> = vec![
//!         "-a",
//!         "-b", "value"
//!         ].into_iter().map(|s| s.to_string()).collect();
//! 
//!     //Parsing of arguments
//!     let result = match parser.parse_custom(example_user_args) {
//!         Err(msg) => panic!("Err: {:?}", msg),//If error occurs panic for example
//!         Ok(v) => {
//!             v.short_options // On success only return the short options
//!         }
//!     };
//!     println!("Result: {:?}", result);
//! }
//! ```
fn main() {
    use arg_parse::config::{self, ShortOption};
    let parser = {
        let config = config::Config::from(&[
            ShortOption{name: 'a', value_count: 0}, // Short Option called a which doesn't accept any arguments
            ShortOption{name: 'b', value_count: 1}, // Short Option called b which accepts one argument
            ShortOption{name: 'c', value_count: 3} // Short Option called c which accepts two arguments
        ], &[/*No long options*/], &[/*No subcommands */]);
        arg_parse::ArgParser::from(config)
    };

    //Some example user inputs fitting 
    let example_user_args: std::vec::Vec<String> = vec![
        "-a",
        "-b", "value"
        ].into_iter().map(|s| s.to_string()).collect();

    //Parsing of arguments
    let result = match parser.parse_custom(example_user_args) {
        Err(msg) => panic!("Err: {:?}", msg),//If error occurs panic for example
        Ok(v) => {
            v.short_options // On success only return the short options
        }
    };
    println!("Result: {:?}", result);
}