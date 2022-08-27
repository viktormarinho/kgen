extern crate kgen;

use kgen::{args_parser, generator, helper};

fn main() {
    let arguments = args_parser::parse();
    helper::show_help(&arguments);
    generator::generate(arguments);
}
