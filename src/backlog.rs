use std::{env, fs};
use include_dir::{include_dir, Dir, File};
use std::path::{Path, PathBuf};
fn main() {
    let args: Vec<String> = env::args().collect();
    //let mut v  = Vec::new();
    let dir_n= "$CARGO_MANIFEST_DIR/src/my_log";
    static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/my_log");
    
    if &args[1] == "show"{
        let var2 = make_path(&args).with_extension("txt");
        let lib_rs =  PROJECT_DIR.get_file(var2).unwrap();
        let content = lib_rs.contents_utf8().unwrap();
        output(content);
    }
    if &args[1] == "-a" || &args[1] == "--about" {
        about();
    }
}

// fn list_files<'k>( file_name: &'k Dir<'_>){
//     let mut v  = Vec::new();
//     for entry in file_name.entries() {
//         println!("{}", entry.path().is_dir());
//         //let md =  Path::new("/home/adudak/gits/imprint/src/my_log/ssh.txt").is_file();
//         println!("{}",file_name);

//         println!("-");
//         v.push(entry);
//     }
// }
fn make_path(path_vec: &Vec<String>) -> PathBuf {
    let mut path_1 = PathBuf::new();
    for i in 2..path_vec.len(){
        path_1.push(&path_vec[i]);
    }
    return path_1;
}

// fn show_command<'k>(path_vec: &Vec<String>, file_name: &'k mut [&File]) -> &'k Path {
//     let mut path_var = String::from("");
//     for i in 2..path_vec.len(){
//             path_var = path_var + &path_vec[i] + "/";
//     }
//     let path_var = &path_var[0..path_var.len() - 1];
//     println!("{}", path_var);
//     for entry in file_name {
//         let path = Path::new(&path_var);
//         let  = path.with_extension("txt");
//         println!("{}", entry.path().display());
//         println!("{}", new_path.display());
//         if new_path == entry.path(){
//             return entry.path();
//         }
//     }
//     return Path::new("");
// }

fn output(output_info: &str){
    println!("=====================================\n");
    println!("{output_info}");
    println!("\n=====================================");
}

fn about(){
    println!(",,,,,,,,,,,,,,,,:+?%#@@@@@@@@#%?+:,,,,,,,,,,,,,,,,
,,,,,,,,,,,,,:*S@@@@#%%????%%#@@@@S*:,,,,,,,,,,,,,
,,,,,,,,,,,;%@@@S*;,,,,::::,,,,;*S@@@%;,,,,,,,,,,,      
,,,,,,,,,:%@@#*:,,;*%#@@@@@@#S*;,,:*#@@%:,,,,,,,,,
,,,,,,,,+#@@*:,:?#@@@S?****%S@@@#?:,:*@@#+,,,,,,,,
,,,,,,,+@@#;,,*@@@%;,,,::::,,,;%@@@*,,;#@@+,,,,,,,
,,,,,,;@@@;,,%@@%:,,+%#@@@@#%+,,:%@@%,,;@@@;,,,,,,
,,,,,,%@@*,,?@@%,,;#@@S?++?S@@#;,,%@@?,,*@@%,,,,,,
,,,,,:@@@:,:@@#:,:@@@+,,;;,,+@@@:,:#@@:,:@@@:,,,,,
,,,,,;@@#,,;@@S,,+@@%,,*@@*,,%@@+,,S@@;,,#@@:,,,,,
,,,,,:#S+,,%@@?,,?@@?,,%@@*,,%@@+,,S@@;,,#@@;,,,,,
,,,,,,:,,;%@@S:,:@@@:,:#@@;,:#@@:,,S@@;,,#@@;,,,,,
,,,,,:?%#@@#*,,;#@@*,,?@@%,,*@@%,,,S@@;,,#@@;,,,,,
,,,,,:##S?+,,:?@@@+,,*@@#:,:#@@;,,,S@@;,,#@@;,,,,,
,,,,,,,,,,:+%@@@%:,:?@@S:,:S@@*,,,,S@@;,,#@@;,,,,,
,,,,,:%%S#@@@#?:,,+#@@%:,:S@@?,,,,,S@@+,,S@@:,,,,,
,,,,,:##S%?+:,,:*#@@S+,,+#@@*,,+?,,*@@%,,;%S:,,,,,
,,,,,,,,,,,:+?S@@@%+,,;%@@S;,,*@@;,,%@@S+:,,,,,,,,
,,,,,,:%S#@@@@#%+:,,+S@@#*,,,,+@@#+,,+S@@@S:,,,,,,
,,,,,,,+SS?*+:,,:+?#@@#*:,:?*,,;#@@%;,,;*%+,,,,,,,
,,,,,,,,,,::;+?S@@@#?;,,;?@@@%:,:?#@@S*;,,,,,,,,,,
,,,,,,,,,:?@@@@#%*;,,:+S@@@%@@#*:,:*S@@%:,,,,,,,,,
,,,,,,,,,,,;+;:,,:;*S@@@S*:,;%@@@?;:,:;,,,,,,,,,,,
,,,,,,,,,,,,,,,+%#@@@S*:,,;+,,;?#@S+,,,,,,,,,,,,,,
,,,,,,,,,,,,,,,::+*+:,,:*S@@%;,,::,,,,,,,,,,,,,,,,\n
                    Imprint
-------------------------------------------------
This is a program for creating your own structure
of notes and using them in the console!");
}
