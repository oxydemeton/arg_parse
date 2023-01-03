use arg_parse::ArgParser;
use arg_parse::config;


const ARGS: &'static [config::Arg] = &[config::Arg::Flag { name: "a" }, config::Arg::Parameter { name: "b" }];
const PARSER_ROOT_CMD: config::Cmd = config::Cmd::from(ARGS, &[]);

static PARSER: ArgParser = ArgParser::from(PARSER_ROOT_CMD);

fn main() {
    let root_cmd = PARSER.parse();
    match root_cmd {
        Ok(result) => println!("Result: {:?}", result),
        Err(error) => println!("ERROR: {:?}", error)
    }
}
