use std::vec;

#[test]
fn main(){
    use arg_parse::{ArgParser, config::{self, LongOption}};
    const parser: ArgParser = {
        let short_options= &[];
        let long_options = &[
            LongOption{name: "a_option", value_count: 0},
            LongOption{name: "b_option", value_count: 0},
            LongOption{name: "c_option", value_count: 1},
            LongOption{name: "d_option", value_count: 2},
            LongOption{name: "e_option", value_count: 3},
            LongOption{name: "f_option", value_count: 1},
            LongOption{name: "g_option", value_count: 5},
            LongOption{name: "h_option", value_count: 3},
            LongOption{name: "i_option", value_count: 0},
            LongOption{name: "j_option", value_count: 1},
        ];
        let config = config::Cmd::from(short_options, long_options, &[]);
        ArgParser::from(config)
    };
    let arguments: Vec<String> = vec![
        "--a_option",
        "--b_option",
        "--c_option", "c_value",
        "--d_option", "d_v_1", "d_v_2",
        "--e_option", "e_v_1", "e_v_2", "e_v_3",
        "--f_option", "f_value",
        "--g_option", "g_v1", "g_v2", "g_v3", "g_v4", "g_v5",
        "--h_option", "h_n1", "h_n2", "h_n3",
        ].into_iter().map(|s| s.to_string()).collect();
    let result = parser.parse_custom(arguments);
    match result {
        Err(msg) => panic!("{:?}", msg),
        Ok(_) => {},
    }
}