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

    pub fn parse(&self, arguments: &[String])->super::result::Cmd {
        let mut result = super::result::Cmd {
            args: vec![],
            sub_cmd: None,
        };

        for a in arguments {
            //Parse flags
            if a.starts_with("--") {
                let name = a.trim_start_matches("--");

                for self_a in self.args {
                    match self_a {
                        Arg::Flag(self_name, _) => {
                            if *self_name == name {
                                result.args.push(super::result::Arg::Flag(self_name, true))
                            }
                        },
                        Arg::Arg(_, _) => todo!(),
                    }
                }

            }

        }
        result        
    }
}