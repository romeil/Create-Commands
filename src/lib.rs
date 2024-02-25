pub mod create {
    use std::fs;
    use std::path::Path;

    pub fn create_file(result: &String) {
        let path = Path::new(result);
        if path.exists() {
            println!("This file already exists.");
        } else {
            println!("\"{}\" was created", result);
            fs::File::create(result)
                        .expect("Error encountered while creating file!");
        }
    }
    
    pub fn create_folder(result: &String) {
        let path = Path::new(result);
        if path.exists() {
            println!("This folder already exists.");
        } else {
            println!("\"{}\" was created", result);
            fs::create_dir(result)
                        .expect("Error encountered while creating folder!");
        }
    }
}