use crate::args_parser::Arguments;
use colored::Colorize;
use crate::helper::show_help;

pub fn throw_error(message: String) -> () {
    crate::helper::show_usage();
    let message = format!("\nERROR: {}", message);
    println!("{}", message.red());
    std::process::exit(1);
}

pub fn validate_arguments(arg_list: &Arguments) -> () {
    if arg_list.len() == 0 {
        show_help(&vec![String::from("--help")])
    }
    if arg_list[0] == "--help" {
        return ();
    }
    if arg_list.len() < 2 {
        throw_error(format!("Please provide at least 2 arguments."))
    }
}
