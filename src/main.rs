use std::env;
use include_dir::{include_dir, Dir, File};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut v  = Vec::new();

    static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/my_log");

    for entry in PROJECT_DIR.files() {
        v.push(entry);
    }

    if &args[1] == "show"{
        let var2 = show_command(&args[2], &mut v);
        let lib_rs =  PROJECT_DIR.get_file(var2).unwrap();
        let content = lib_rs.contents_utf8().unwrap();
        output(content);
    }
    if &args[1] == "-a" || &args[1] == "--about" {
        about();
    }
}

fn show_command<'k>(command_name: &str, file_name: &'k mut [&File]) -> &'k Path {
    for entry in file_name {
        let path = Path::new(command_name);
        let new_path = path.with_extension("txt");

        if new_path == entry.path(){
            return entry.path();
        }
    }
    return Path::new("");
}


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