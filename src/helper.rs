use crate::args_parser::Arguments;
use colored::Colorize;

pub fn show_usage() -> () {
    println!("==============================================");
    println!(
        "{} {} {}",
        "USAGE:".white(),
        "kgen".yellow(),
        "[FILE TYPE] [NAME] <EXTRA OPTIONS>".yellow().bold()
    );
    println!(
        "{} {}",
        "EXAMPLE:".white(),
        "kgen component footer".yellow().italic()
    );
}

pub fn show_help(args: &Arguments) -> () {
    if args[0] == "--help" {
        show_usage();
        println!("\n{}\n", "KGEN HELP".blue().bold().underline());
        println!(" - {}", "FILE TYPES:".blue().bold());
        println!("   {} | alias: {} | Generates a component file with boilerplate in the components directory.", "component".yellow(), "c".yellow());
        println!("   {} | alias: {} | Generates a page component file with boilerplate in the pages directory.", "page".yellow(), "p".yellow());
        println!("\n - {}", "OPTIONS:".blue().bold());
        println!(
            "   {} | alias: {} | Adds data fetching logic to the component.",
            "--data".yellow(),
            "-d".yellow()
        );
        println!(
            "   {} | alias: {} | Creates the component in the current directory.",
            "--local".yellow(),
            "-l".yellow()
        );
        println!(
            "   {} | alias: {} | Zimbas.",
            "--zimbas".yellow(),
            "-z".yellow()
        );
        println!("\n");

        std::process::exit(0);
    }
}
