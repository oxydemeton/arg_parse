use std::vec;

#[test]
fn main(){
    use arg_parse::{ArgParser, config::{self, ShortOption}};
    const parser: ArgParser = {
        let short_options: &'static [ShortOption] = &[
            ShortOption{name:'a', value_count: 0},
            ShortOption{name:'b', value_count: 0},
            ShortOption{name:'c', value_count: 1},
            ShortOption{name:'d', value_count: 2},
            ShortOption{name:'e', value_count: 3},
            ShortOption{name:'f', value_count: 1},
            ShortOption{name:'g', value_count: 5},
            ShortOption{name:'h', value_count: 3},
            ShortOption{name:'i', value_count: 0},
            ShortOption{name:'j', value_count: 1},
            ];
        let long_options = &[];
        let config = config::Cmd::from(short_options, long_options, &[]);
        ArgParser::from(config)
    };
    let arguments: Vec<String> = vec![
        "-a",
        "-b",
        "-c", "c_value",
        "-d", "d_v_1", "d_v_2",
        "-e", "e_v_1", "e_v_2", "e_v_3",
        "-f", "f_value",
        "-g", "g_v1", "g_v2", "g_v3", "g_v4", "g_v5",
        "-h", "h_n1", "h_n2", "h_n3",
        ].into_iter().map(|s| s.to_string()).collect();
    let result = parser.parse_custom(arguments);
    match result {
        Err(msg) => panic!("{:?}", msg),
        Ok(_) => {},
    }
}