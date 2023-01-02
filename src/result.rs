#[derive(Debug, Clone)]
pub enum Arg {
    //Name, set or not set
    Flag(&'static str, bool),
    //Name, Value
    Parameter(&'static str, Option<String>)
}


#[derive(Debug)]
pub struct Cmd {
    pub args: Vec<Arg>,
    pub sub_cmd: Option<Box<Cmd>>
}