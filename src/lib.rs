//! # arg_parse for rust
//! ### Description
//! arg_parse is a tool to simplify the processing of command line arguments. It doesn't have any dependencies and the initialization is done at compile time.
//!  
//! ## Features
//! - [x] Parsing of `flags` (Values set with `--` which default is false and set to true by being used.)
//! - [x] Parsing of `parameters` (Values mentioned after `-` which have their value(as a string) followed)
//! - [ ] Parsing of `sub commands` (which only one can be used and all following arguments are related to)
//! - [ ] Returning results instead of throwing unfinished error messages
//! - [ ] Simple creation of parser
//! - [x] Ability to create parser as constant or static variable (at compile time)
//! - [x] Ability to give a list of arguments (not using args from [std::env::args()](std::env::args()))
//! - [ ] Ability to provide default values
//! - [ ] Ability to make Argument or subcommand required
//! - [ ] Cache the result of parsing the cli arguments to improve performance slightly
//! - [ ] Easy macro or function to configure the parser
//! - [ ] Split Config and Results into different features which can be enabled or disabled in the cargo.toml
//! - [ ] fulfill common patters, like described in this [specification](https://gist.github.com/pksunkara/1485856)
//! 
//! # Installation
//! Add `arg_parse = "0.1.0"` to your cargo dependencies (`cargo.toml`).
//! ```toml
//! [dependencies]
//! arg_parse = "0.1.0"
//! ```
//! # Example
//! Prints if the flag `--a` is provided and the parameter provided under `-b`
//! ```rust
//! use arg_parse::ArgParser;
//! use arg_parse::config;
//! // Define all Arguments of the program itself/root command (compile time)
//! const ARGS: &'static [config::Arg] = &[config::Arg::Flag("a"), config::Arg::Parameter("b")];
//! // Define the Root Command, without any possible sub commands (compile time)
//! const PARSER_ROOT_CMD: config::Cmd = config::Cmd::from(ARGS, &[]);
//! // Create the Parser in static memory, available everywhere (Created at compile time)
//! static PARSER: ArgParser = ArgParser::from(PARSER_ROOT_CMD);
//! 
//! fn main() {
//!     //Parse the by the user provided Arguments
//!     let root_cmd = PARSER.parse();
//!     match root_cmd {
//!         Ok(result) => println!("Result: {:?}", result),
//!         Err(error) => println!("ERROR: {:?}", error)
//!     }
//! }
//! ```
//! ### Links:
//! [Github Repo](https://github.com/oxydemeton/arg_parse/)<br>
//! [Crates.io](https://crates.io/crates/arg_parse)<br>
//! [Rust Docs](https://docs.rs/arg_parse/latest/arg_parse/)
 

/// Structs and Enums to configure the parser
pub mod config;
/// Structs and Enums the parser return as a Result
pub mod result;
/// Tools to parse Arguments
pub mod parser;
pub use parser::ArgParser;