use std::fs;
use std::io::ErrorKind;

struct CreatedFile {
    file_name: String,
    file_body: String,
}

struct Directory {
    directory_name: String,
    subdirectories: Vec<Directory>,
    files: Vec<CreatedFile>,
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

        for subfile in &mut self.files {
            let CreatedFile { file_name, file_body } = subfile;

            let new_file_name = format!("{self_directory_name}/{file_name}");
            let new_file_body = format!("{file_body}\n");
            fs::write(new_file_name, new_file_body).unwrap();
        }
    
        for subdirectory in &mut self.subdirectories {
            let directory_name = &mut subdirectory.directory_name;
            let new_directory_name = format!("{self_directory_name}/{directory_name}");
            subdirectory.directory_name = new_directory_name;
            subdirectory.generate()
        }
    }
}

fn main() {
    let mut dir = Directory {
        directory_name: String::from("Test"),
        subdirectories: Vec::new(),
        files: Vec::new(),
    };

    dir.subdirectories.push(Directory {
        directory_name: String::from(":3"),
        subdirectories: Vec::new(),
        files:  vec![CreatedFile { file_name: String::from("test.txt"), file_body: String::from("deez") }],
    });

    dir.files.push(CreatedFile { file_name: String::from("hello"), file_body: String::from("helloooo") });
    dir.generate();
}
