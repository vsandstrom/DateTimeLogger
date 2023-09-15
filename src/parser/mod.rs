use clap::Parser;
use crate::validate::NameOrBool;


#[derive(Parser, Default, Debug)]
pub struct Cli {
    #[clap(short, long)]
    // #[clap(default_value_t=String::from("Default Defaultsson"))]
    /// Name of user 
    pub name: Option<String>,
    #[clap(short, long)]
    #[clap(default_value_t=String::from("127.0.0.1"))]
    /// IP adress for websocket to attach to
    pub ip: String,
    #[clap(short, long)]
    #[clap(default_value_t=String::from("6666"))]
    /// Port for websocket to listen to
    pub port: String,
    #[clap(short, long, takes_value=false)]
    #[clap()]
    /// Determines local or remote data input
    pub websocket: bool,
}

pub fn populate_arg_variables(arg: Option<String>) -> NameOrBool {
    let a: NameOrBool = match arg {
        Some(arg) => {
            NameOrBool::new(arg,true)
        },
            // NameOrBool::name = String::from(arg)},
        None => {
            NameOrBool::new(String::from(""),false)
        }
    };
    return a;
}
