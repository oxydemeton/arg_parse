use std::vec;
#[derive(Debug, Clone)]
pub enum Arg {
    //Name, required
    Flag(&'static str, bool),
    //Name, required
    Arg(&'static str, bool)
}
#[derive(Debug)]
pub struct Cmd {
    pub args: &'static[Arg],
    pub sub_cmd: &'static[Cmd]
}

impl Cmd {
    pub const fn new()->Self {
        Self { args: &[], sub_cmd: &[]}
    }
    
    pub const fn from(args: &'static[Arg], sub_cmd: &'static[Cmd])->Self {
        Self { args, sub_cmd}
    }
    //Parse a command from a bunch of words
    pub fn parse(&self, arguments: &[String])->super::result::Cmd {
        let mut result = super::result::Cmd {
            args: vec![],
            sub_cmd: None,
        };

        let mut skip = 0;        
        for (i, a) in arguments.iter().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if a.starts_with("--") { //Flags
                let name = a.trim_start_matches("--");
                match self.find_arg(name) {
                    Some(a) => match a {
                        Arg::Flag(self_name, _) => result.args.push(super::result::Arg::Flag(self_name, true)),
                        Arg::Arg(_, _) => todo!("Error message not implemented correctly: {} is an Argument, not a flag.", name),
                    },
                    None => todo!("Error Message not implemented correctly: Unknown argument: {}", name),
                }
                
            }else if a.starts_with("-") { // Arguments
                let name = a.trim_start_matches("-");
                match self.find_arg(name) {
                    Some(a) => match a {
                        Arg::Arg(self_name, _) => {
                            if i+1 >= arguments.len() {
                                todo!("Error handling not implemented: Argument without parameter.");
                            }
                            let value = arguments[i+1].clone();
                            skip +=1;
                            result.args.push(super::result::Arg::Arg(self_name, Some(value)))
                        },
                        Arg::Flag(_, _) => todo!("Error message not implemented correctly: {} is an Argument, not a flag.", name),
                    },
                    None => todo!("Error Message not implemented correctly: Unknown argument: {}", name),
                }
            } else { //Subcommand
                todo!("Subcommands not implemented")
            }

        }
        result        
    }

    fn find_arg(&self, name: &str)-> Option<&Arg> {
        for self_a in self.args {
            match self_a {
                Arg::Flag(self_name, _) => {
                    if *self_name == name {
                        return Some(self_a);
                    }
                },
                Arg::Arg(self_name, _) => {
                    if *self_name == name {
                        return Some(self_a);
                    }
                },
            }
        }
        None
    }
}