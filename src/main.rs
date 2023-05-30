use std::{env};
use include_dir::{include_dir, Dir};
use std::path::{PathBuf};
mod main_info;

struct ReadPathError{details: String}

impl ReadPathError {
    fn new(msg: &str) -> ReadPathError {
        ReadPathError{details: msg.to_string()}
    }
}

fn read_note<'k>(args_vec: &Vec<String>, project_dir: &Dir<'static>) -> Result<&'k str, ReadPathError> {
    let mut path_to_note = PathBuf::new();
    for i in 2..args_vec.len(){
        path_to_note.push(&args_vec[i]);
    }
    let path_file_note = path_to_note.with_extension("txt");
    if project_dir.get_file(&path_file_note).is_some() == false{
        Err(ReadPathError::new("\x1b[35mOoops\x1b[0m\nLooks like this note is not exist"))
    } else{
        let file_note =  project_dir.get_file(&path_file_note).unwrap();
        let content = file_note.contents_utf8().unwrap();
        Ok(content)
    }
}
fn main() {
    env::set_var("RUST_BACKTRACE", "0");
    let args: Vec<String> = env::args().collect();
    static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/my_log");

    let first_arg: &str= &args[1];

    match first_arg {
        "print" => {
            match read_note(&args, &PROJECT_DIR){
                Ok(content) => output(content),
                Err(error) => println!("{}", error.details)
            };
    },
        "-a" | "--about" => main_info::about(),
        "-v" | "--version" => main_info::version(),
        "-h" | "--help" => main_info::help(),
        _ => println!("\x1b[31mError\x1b[0m\nSomeething wrong with first parameter\nTry use parameter \"print\" or see info with parametr \"-h\"")
    }
}


fn output(output_info: &str){
    println!("=====================================\n");
    println!("{output_info}");
    println!("\n=====================================");
}

