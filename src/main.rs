use arg_parse::ArgParser;
use arg_parse::config;

const LONG_OPTIONS: &'static [config::LongOption] = &[
    config::LongOption{name: "hello", value_count: 0}
    ];
const SHORT_OPTIONS: &'static [config::ShortOption] = &[
    config::ShortOption{name:'b', value_count: 2},
    config::ShortOption{name:'a', value_count: 0}
    ];
const PARSER_ROOT_CMD: config::Cmd = config::Cmd::from(SHORT_OPTIONS, LONG_OPTIONS, &[]);

static PARSER: ArgParser = ArgParser::from(PARSER_ROOT_CMD);

fn main() {
    let root_cmd = PARSER.parse();
    assert!(matches!(root_cmd, Ok(_)));
    match root_cmd {
        Ok(result) => println!("Result: {:?}", result),
        Err(error) => println!("ERROR: {:?}", error)
    }
}