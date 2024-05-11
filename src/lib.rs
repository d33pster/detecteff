use rustypath::RPath;

pub struct File {
    path: RPath,
    duplicates: Vec<RPath>,
}

pub struct Files {
    files: Vec<File>,
}

impl Files {
    pub fn new(path: RPath, recursive: bool) -> Self {
        if recursive {
            Self {
                files: Files::scan_recursive(&path),
            }
        } else {
            Self{
                files: Files::scan(&path),
            }
        }
    }

    fn scan_recursive(path: &RPath) -> Vec<File> {
        let mut file_rpaths: Vec<File> = Vec::new();
    
        if let Ok(entries) = std::fs::read_dir(path.convert_to_pathbuf()) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.is_dir() {
                    // If the entry is a directory, recursively call scan on it
                    let subdirectory_files = Self::scan(&RPath::new_from_pbuf(&file_path));
                    for sub_file in subdirectory_files {
                        file_rpaths.push(sub_file);
                    }
                } else {
                    // If the entry is a file, add it to the list of files
                    if let Some(file_name) = entry.file_name().to_str() {
                        file_rpaths.push(File {
                            path: path.join(&file_name),
                            duplicates: Vec::new(),
                        });
                    }
                }
            }
        }
    
        file_rpaths
    }

    fn scan(path: &RPath) -> Vec<File> {
        let mut file_rpaths: Vec<File> = Vec::new();
    
        if let Ok(entries) = std::fs::read_dir(path.convert_to_pathbuf()) {
            for entry in entries.filter_map(Result::ok) {
                let metadata = entry.metadata().ok();
                if let Some(metadata) = metadata {
                    if metadata.is_file() {
                        if let Some(_file_name) = entry.file_name().to_str() {
                            let file_path = entry.path();
                            file_rpaths.push(File {
                                path: RPath::new_from_pbuf(&file_path),
                                duplicates: Vec::new(),
                            });
                        }
                    }
                }
            }
        }
    
        file_rpaths
    }

    pub fn find_duplicates(&mut self) {
        let mut done: Vec<String> = Vec::new();
        for i in 0..=self.files.len()-1 {
            if !done.contains(&self.files[i].path.convert_to_string()) {
                println!("Scanning {}", self.files[i].path.convert_to_string());
                // read each file's content and check with all other files except itself.
                let content = std::fs::read(self.files[i].path.convert_to_pathbuf().clone())
                    .expect(&("Unable to read ".to_string()+&self.files[i].path.convert_to_string()));

                done.push(self.files[i].path.clone().convert_to_string());

                // let mut title_count = 0;
                // let mut filecounts = 1;

                for j in 0..=self.files.len()-1 {
                    if j == i {
                        continue;
                    } else {
                        // read content
                        let content2 = std::fs::read(self.files[j].path.convert_to_pathbuf())
                            .expect(&("Unable to read ".to_string()+&self.files[j].path.convert_to_string()));

                        
                        if content == content2 {
                            let f = self.files[j].path.clone();
                            done.push(f.convert_to_string());

                            self.files[i].duplicates.push(f);

                            // if title_count == 0 {
                            //     println!("{}:", self.files[i].path.convert_to_string());
                            //     title_count += 1;
                            // }

                            // println!("{}. {}", filecounts, self.files[j].path.convert_to_string());
                            // filecounts += 1;
                        }
                    }
                }
            }
        }
    }

    pub fn show(&self) {
        for file in &self.files {
            if file.duplicates.len() > 0 {
                let mut string = file.path.convert_to_string() + " -> {";

                let mut count = 0;

                for d in &file.duplicates {
                    if count > 0 {
                        string = string + ", " + &d.convert_to_string();
                    } else {
                        string = string + " " + &d.convert_to_string();
                    }
                    count += 1;
                }

                string = string + " }";

                if string != file.path.convert_to_string() + " ->{ }" {
                    println!("{}", string);
                }
            }
        }
    }
}
