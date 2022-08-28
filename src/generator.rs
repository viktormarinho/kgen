use crate::{args_parser::Arguments, error_handler::throw_error};
use std::fs::File;

pub enum FileType {
    Component,
    Page,
}

#[derive(PartialEq)]
pub enum Flag {
    Data,
    Local,
    Children,
}

pub struct FileToGen {
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

fn get_flag_type(flag: &str) -> Option<Flag> {
    match flag {
        "--data" | "-d" => Some(Flag::Data),
        "--local" | "-l" => Some(Flag::Local),
        "--children" | "-c" => Some(Flag::Children),
        _ => {
            throw_error(format!(
                "Flag {} does not exist.\nUse kgen --help to see all available flags.",
                flag
            ));
            None
        }
    }
}

fn validate_flags(flag_list: &Vec<String>) -> Vec<Flag> {
    let mut validated_flag_list = Vec::<Flag>::new();
    for flag in flag_list.iter() {
        validated_flag_list.push(get_flag_type(flag).unwrap());
    }

    validated_flag_list
}

fn get_flags(arg_list: &mut Arguments) -> Vec<Flag> {
    if arg_list.len() == 2 {
        return Vec::<Flag>::new();
    }
    arg_list.remove(0);
    arg_list.remove(0);
    let arg_list = validate_flags(&arg_list);

    arg_list
}

fn create_file(file: FileToGen, flags: Vec<Flag>) {

    // NEED TO MAKE CORRECT PATH SELECTOR HERE.
    let mut path = String::new();

    if flags.contains(&Flag::Local) {
        path = String::from("./")
    }

    let mut file_ref = match File::create(format!("{}{}", path, file.name)) {
        Ok(file_ref) => file_ref,
        Err(_) => {
            throw_error(format!(
                "could not create file {} in the specified path '{}'.",
                file.name,
                format!("{}{}", path, file.name)
            ));
            // This code will never run
            File::create("").unwrap()
        }
    };

    let text = template_parser::get_full_text(&file, &flags);
}

mod template_parser {
    use super::{FileToGen, FileType, Flag};
    use crate::hardcoded::{DATA_REPLACERS, DATA_VARS, CHILDREN_REPLACERS, CHILDREN_VARS};

    const NOTHING: &str = "";

    fn insert_vars(full_text: String, replacers: &[&str], vars: &[&str]) -> String {
        let mut full_text = full_text;

        for (index, replacer) in replacers.iter().enumerate() {
            full_text = full_text.replace(vars[index], replacer);
        }

        full_text
    }

    fn clear_vars(full_text: String, vars: &[&str]) -> String {
        let mut full_text = full_text;

        for var in vars.iter() {
            full_text = full_text.replace(var, NOTHING);
        }

        full_text
    }

    pub fn get_full_text(file: &FileToGen, flags: &Vec<Flag>) -> String {
        let full_text = match file.file_type {
            FileType::Component => include_str!("./bases/Component.in"),
            FileType::Page => include_str!("./bases/Component.in"),
        };

        let mut full_text = full_text.replace("%COMPONENT_NAME%", file.name.as_str());

        full_text = match flags.contains(&Flag::Data) {
            true => insert_vars(full_text, &DATA_REPLACERS, &DATA_VARS),
            false => clear_vars(full_text, &DATA_VARS),
        };

        full_text = match flags.contains(&Flag::Children) {
            true => insert_vars(full_text, &CHILDREN_REPLACERS, &CHILDREN_VARS),
            false => clear_vars(full_text, &CHILDREN_VARS),
        };

        println!("{}", full_text);

        full_text
    }
}
