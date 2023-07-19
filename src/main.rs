use std::fs;
use std::io::ErrorKind;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct CreatedFile {
    file_name: String,
    file_body: String,
}

#[derive(Deserialize)]
struct Directory {
    directory_name: String,
    subfolders: Option<Vec<Directory>>,
    files: Option<Vec<CreatedFile>>,
}

#[derive(Deserialize)]
struct Project {
    directories: Vec<Directory>,
    files: Option<Vec<CreatedFile>>,
}

fn write_file(file_body: &String, file_name: &String) {
    let new_file_body = format!("{file_body}\n");
    fs::write(file_name, new_file_body).unwrap()
}

impl Directory {
    fn generate(&mut self) {
        let self_directory_name = &self.directory_name;
        
        let result = fs::create_dir(self_directory_name);

        match result {
            Ok(_) => {},
            Err(error) => match error.kind() {
                ErrorKind::AlreadyExists => {},
                _ => {
                    panic!("There was an error writing to the file!");
                }
            },
        };
        

        match &mut self.files {
            Some(files) => {
                for subfile in files {
                    let CreatedFile { file_name, file_body } = subfile;

                    let new_file_name = format!("{self_directory_name}/{file_name}");
                    write_file(&file_body, &new_file_name);
                }
            },

            None => {},
        };
   
        match &mut self.subfolders {
            Some(subfolders) => {
                for subdirectory in subfolders {
                    let directory_name = &mut subdirectory.directory_name;
                    let new_directory_name = format!("{self_directory_name}/{directory_name}");
                    subdirectory.directory_name = new_directory_name;
                    subdirectory.generate()
                }
            },

            None => {}, 
        };
    }
}

impl Project {
    fn generate(&mut self) {
        match &mut self.files {
            Some(files) => {
                for file in files {
                    write_file(&file.file_body, &file.file_name)
                }
            }

            None => {},
        };

        for directory in &mut self.directories {
            directory.generate();
        }
    }
}

fn get_cli_path() -> String {
    let exe_dir = env::current_exe().unwrap();
    let exe_dir = exe_dir.to_str();

    match exe_dir {
        Some(dir) => return String::from(dir),
        None => panic!("Error while reading the directory!"),
    };
}

fn main() {


    let contents = fs::read_to_string("templates/C.toml").unwrap();
    let mut project: Project = toml::from_str(&contents).unwrap();
    project.generate();
    let exe_dir = get_cli_path();
    println!("{exe_dir}");
}
