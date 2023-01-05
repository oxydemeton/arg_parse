use arg_parse::ArgParser;
use arg_parse::config;

const LONG_OPTIONS: &'static [config::LongOption] = &[
    config::LongOption{name: "hello", value_count: 1}
];
const SHORT_OPTIONS: &'static [config::ShortOption] = &[
    config::ShortOption{name:'b', value_count: 0},
    config::ShortOption{name:'a', value_count: 1}
];
const NON_OPTIONS: &'static [config::NonOption] = &[
    config::NonOption{name:"abc", value_count: 1}
];
const PARSER_ROOT_CMD: config::Config = config::Config::from(SHORT_OPTIONS, LONG_OPTIONS, NON_OPTIONS);

static PARSER: ArgParser = ArgParser::from(PARSER_ROOT_CMD);

fn main() {
    let root_cmd = PARSER.parse();
    match root_cmd {
        Ok(result) => println!("Result: {:?}", result),
        Err(error) => println!("ERROR: {:?}", error)
    }
}