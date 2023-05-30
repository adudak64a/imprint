use include_dir::{Dir};

pub fn list_note(project_dir: &Dir){
        for entry in project_dir.entries(){
                if entry.path().extension().is_some() && entry.path().extension().unwrap() == "txt"{
                        println!("{}", entry.path().display())
                }
        }
        for dirs in project_dir.dirs(){
                list_note(dirs);
        }
}
