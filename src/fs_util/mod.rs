use std::fs;
pub fn read_dir(directory: &str) {
    let dir = fs::read_dir(directory).expect("Could not read dir");

    for item in dir {
        match item {
            Ok(entry) => {
                println!("Found [{:?}]", entry);
            }
            Err(e) => {
                eprintln!("[Error]: Could not find dirEntry => {e}");
            }
        }
    }
}