#[test]
fn main() {
    use arg_parse::{ArgParser, config};
    let _ = {
        let short_options = &[];
        let long_options = &[];
        let config = config::Cmd::from(short_options, long_options, &[]);
        ArgParser::from(config)
    };
}