# arg_parse for rust
# Disclaimer
The interface for developer is work in progress. So expect minor and major changes when updating until v1.0<br>
# Description
arg_parse is a tool to simplify the processing of command line arguments. It doesn't have any dependencies and the initialization is done at compile time.<br>

# Features & Goals
- [x] Parsing of `short options` (Values set with `--` which default is false and set to true by being used.)
- [x] Parsing of `long options` (Values mentioned after `-` which have their value(as a string) followed)
- [ ] Parsing of `non options` (Single Values parameters without any prefix )
- [ ] Parsing of `sub commands` (which only one can be used and all following arguments are related to)
- [x] Returning results instead of throwing unfinished error messages
- [ ] Simple creation of parser
- [x] Ability to create parser as constant or static variable (at compile time)
- [x] Ability to give a list of arguments (not using args from [std::env::args()](std::env::args()))
- [ ] Ability to provide default values
- [ ] Ability to make Argument or subcommand required
- [ ] Cache the result of parsing the cli arguments to improve performance slightly
- [x] fulfill  common patters, like described in this [specification](https://gist.github.com/pksunkara/1485856)

# Installation
#### Only version 0.1.1 is published yet!
Add `arg_parse = "0.1.1"` to your cargo dependencies (`cargo.toml`).
```toml
[dependencies]
arg_parse = "0.1.1"
```

 # Example
 Prints the Options which are found or parsing errors.<br>
 Arguments:
 - LongOption: `hello` without any further arguments
 - ShortOption: `b` with two arguments
 - ShortOption: `a` without any arguments
 ```rust
use arg_parse::ArgParser;
use arg_parse::config;

//List of all available long options
const LONG_OPTIONS: &'static [config::LongOption] = &[
    //Define a long option called hello without parameters
    config::LongOption{name: "hello", value_count: 0}
    ];
//List of all available short options
const SHORT_OPTIONS: &'static [config::ShortOption] = &[
    //Define a short option called b with two parameters
    config::ShortOption{name:'b', value_count: 2},
    //Define a short option called a without parameters
    config::ShortOption{name:'a', value_count: 0}
    ];

//Create the root command which is the program itself basically
const PARSER_ROOT_CMD: config::Cmd = config::Cmd::from(SHORT_OPTIONS, LONG_OPTIONS, &[]);

//Create the parser from the root command
static PARSER: ArgParser = ArgParser::from(PARSER_ROOT_CMD);

fn main() {
    let root_cmd = PARSER.parse(); //Parse the command line arguments
    match root_cmd {
        Ok(result) => println!("Result: {:?}", result), //Print result
        Err(error) => println!("ERROR: {:?}", error) //Print errors if occur
    }
}
 ```

### Links:
[Github Repo](https://github.com/oxydemeton/arg_parse/) <br>
[Crates.io](https://crates.io/crates/arg_parse)<br>
[Rust Docs](https://docs.rs/arg_parse/latest/arg_parse/)
 