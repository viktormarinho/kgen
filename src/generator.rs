use crate::error_handler::throw_error;

enum FileTypes {
    Component,
    Page,
}

struct FileToGen {
    name: String,
    file_type: FileTypes,
}

fn get_file_type(type_input: &str) -> Option<FileTypes> {
    match type_input {
        "component" | "c" => Some(FileTypes::Component),
        "pages" | "p" => Some(FileTypes::Page),
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

pub fn generate(arg_list: Vec<String>) -> () {
    let file = FileToGen {
        name: format_name(arg_list[1].as_str()),
        file_type: get_file_type(arg_list[0].as_str()).unwrap(),
    };

    create_file(file);
}

fn create_file(file: FileToGen) {}
