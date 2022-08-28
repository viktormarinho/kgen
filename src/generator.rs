use crate::{error_handler::throw_error, args_parser::Arguments};


enum FileType {
    Component,
    Page,
}

enum Flag {
    Data,
    Local,
}

struct FileToGen {
    name: String,
    file_type: FileType,
}

fn get_file_type(type_input: &str) -> Option<FileType> {
    match type_input {
        "component" | "c" => Some(FileType::Component),
        "pages" | "p" => Some(FileType::Page),
        _ => {
            throw_error(format!(
                "First argument {} is not valid.\nUse kgen --help to see all available arguments.",
                type_input
            ));
            None
        }
    }
}

fn format_name(name_in: &str) -> String {
    name_in[0..1].to_uppercase() + &name_in[1..]
}

pub fn generate(arg_list: Arguments) -> () {
    let file = FileToGen {
        name: format_name(arg_list[1].as_str()),
        file_type: get_file_type(arg_list[0].as_str()).unwrap(),
    };

    let flags = get_flags(&mut arg_list.clone());

    create_file(file, flags);
}

fn get_flag_type(flag: &str) -> Option<Flag>{
    match flag {
        "--data" | "-d" => Some(Flag::Data),
        "--local" | "-l" => Some(Flag::Local),
        _ => {
            throw_error(format!(
                "Flag {} does not exist.\nUse kgen --help to see all available flags.",
                flag
            ));
            None
        },
    }
}

fn validate_flags(flag_list: &Vec<String>) -> Vec<Flag> {
    let mut validated_flag_list = Vec::<Flag>::new();
    for flag in flag_list.iter() {
        validated_flag_list.push(get_flag_type(flag).unwrap());
    }

    validated_flag_list
}

fn get_flags(arg_list: &mut Arguments) -> Vec<Flag>{
    if arg_list.len() == 2 {
        return Vec::<Flag>::new();
    }
    arg_list.remove(0);
    arg_list.remove(0);
    let arg_list = validate_flags(&arg_list);

    arg_list
}

fn create_file(file: FileToGen, flags: Vec<Flag>) {

}
