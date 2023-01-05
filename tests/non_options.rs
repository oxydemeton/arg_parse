use std::vec;

#[test]
fn main(){
    use arg_parse::{ArgParser, config::{self, NonOption}};
    let parser: ArgParser = {
        let non_options = &[
            NonOption{name: "a_option", value_count: 0},
            NonOption{name: "b_option", value_count: 0},
            NonOption{name: "c_option", value_count: 1},
            NonOption{name: "d_option", value_count: 2},
            NonOption{name: "e_option", value_count: 3},
            NonOption{name: "f_option", value_count: 1},
            NonOption{name: "g_option", value_count: 5},
            NonOption{name: "h_option", value_count: 3},
            NonOption{name: "i_option", value_count: 0},
            NonOption{name: "j_option", value_count: 1},
        ];
        let config = config::Config::from(&[], &[], non_options);
        ArgParser::from(config)
    };
    let arguments: Vec<String> = vec![
        "a_option",
        "b_option",
        "c_option", "c_value",
        "d_option", "d_v_1", "d_v_2",
        "e_option", "e_v_1", "e_v_2", "e_v_3",
        "f_option", "f_value",
        "g_option", "g_v1", "g_v2", "g_v3", "g_v4", "g_v5",
        "h_option", "h_n1", "h_n2", "h_n3",
        ].into_iter().map(|s| s.to_string()).collect();
    let result = parser.parse_custom(arguments);
    match result {
        Err(msg) => panic!("{:?}", msg),
        Ok(_) => {},
    }
}