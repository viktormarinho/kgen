use crate::error_handler;
use std::env::args;

pub type Arguments = Vec<String>;

pub fn parse() -> Arguments {
    let mut arg_list: Arguments = args().collect();
    arg_list.remove(0);
    error_handler::validate_arguments(&arg_list);

    arg_list
}
