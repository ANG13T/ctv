extern crate dotenv;
use dotenv::dotenv;
use std::{fs, env};
use structopt::StructOpt;
use std::path::{PathBuf};
use std::error::Error;
use crate::protocols::{File};
use crate::input;
use crate::protocols::file::{FileStyle};

#[derive(Clone)]
pub struct TreeGenerator {
    root_dir: PathBuf,
    tree: Vec<String>,
    pipe: String,
    elbow: String, 
    tee: String,
    pipe_prefix: String,
    space_prefix: String,
    show_dir_metadata: String,
    show_file_metadata: String,
    file_styles: FileStyle
}

impl TreeGenerator {
    pub fn init(root_dir: PathBuf) -> Self {
        dotenv().ok();
        let file_style: FileStyle = FileStyle::new(
            env::var("FILE_SIZE_POSITION").unwrap().parse::<i32>().unwrap(),
            env::var("FILE_OWNER_POSITION").unwrap().parse::<i32>().unwrap(),
            env::var("FILE_PERMS_POSITION").unwrap().parse::<i32>().unwrap(),
            env::var("DIR_NAME_COLOR").unwrap(),
            env::var("FILE_NAME_COLOR").unwrap(),
            env::var("FILE_TIME_COLOR").unwrap(),
            env::var("FILE_SIZE_COLOR").unwrap(),
            env::var("FILE_OWNER_COLOR").unwrap(),
            env::var("FILE_PERMS_COLOR").unwrap(),
            env::var("DIR_NAME_STYLE").unwrap(),
            env::var("FILE_NAME_STYLE").unwrap(),
            env::var("FILE_SIZE_STYLE").unwrap(),
            env::var("FILE_OWNER_STYLE").unwrap(),
            env::var("FILE_PERMS_STYLE").unwrap(),
            env::var("FILE_TIME_STYLE").unwrap(),
        );

        Self {
            tree: Vec::new(),
            pipe: env::var("PIPE").unwrap(),
            elbow: env::var("ELBOW").unwrap(),
            tee: env::var("TEE").unwrap(),
            pipe_prefix: env::var("PIPE_PREFIX").unwrap(),
            space_prefix: env::var("SPACE_PREFIX").unwrap(),
            root_dir: root_dir,
            show_dir_metadata: env::var("SHOW_DIR_METADATA").unwrap(),
            show_file_metadata: env::var("SHOW_FILE_METADATA").unwrap(),
            file_styles: file_style
        }   
    }
    pub fn build_tree(&mut self) -> Vec<String>{
        self.tree_head();
        self.tree_body(self.root_dir.clone(), &"".to_string());
        return self.tree.clone();
    }

    fn sort_dir_first(&self, directory: PathBuf) -> Result<Vec<fs::DirEntry>, Box<dyn Error>>{
        let mut dir_vec: Vec<fs::DirEntry> = Vec::new();
        let mut file_vec: Vec<fs::DirEntry> = Vec::new(); 
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
    
            let metadata = fs::metadata(&path)?;
    
            if metadata.is_file(){
                file_vec.push(entry);
            }else if metadata.is_dir(){
                dir_vec.push(entry);
            }
        }

        dir_vec.append(&mut file_vec);
        Ok(dir_vec)
    }

    fn get_dir_item_amount(&self, directory: PathBuf) -> usize {
        return directory.iter().count();
    }

    fn tree_head(&mut self) {
        let dir_file = File::new(self.root_dir.clone(), input::Cli::from_args().created_time.to_string(), &self.file_styles);
        self.tree.push(dir_file.display_format()); // prints out head dir
        self.tree.push(self.pipe.clone()); //print pipe under head dir
    }

    fn tree_body(&mut self, directory: PathBuf, prefix: &String) {

        let entries = self.sort_dir_first(directory).unwrap();
        let entries_count = entries.len();
        

        for (index, entry) in entries.iter().enumerate(){
            let connector;
            let metadata = fs::metadata(entry.path()).unwrap();

            if index == entries_count - 1 && (!metadata.is_dir() ||  self.get_dir_item_amount(entry.path()) == 0) {
                connector = &self.elbow;
            }else{
                connector = &self.tee;
            }

            if metadata.is_dir() {
                self.add_directory(
                    entry.path(), entry.path(), index, entries_count, prefix.to_string(), connector.to_string()
                )
            }else {
                self.add_file(entry.path(), prefix.to_string(), connector.to_string())
            }
        }   
    }

    fn add_directory(&mut self, directory: PathBuf, directory2: PathBuf, index: usize, entries_count: usize, mut prefix: String, connector: String) {
        let new_file = File::new(directory, input::Cli::from_args().created_time.to_string(), &self.file_styles);
        let file_name = if self.show_dir_metadata == "TRUE" {new_file.display_format()} else {new_file.get_name()};
        self.tree.push(format!("{}{} {}", prefix, connector, file_name));
        if index != entries_count - 1 {
            prefix += &self.pipe_prefix;
        }else {
            prefix += &self.space_prefix;
        }
            
        self.tree_body(directory2, &prefix.to_string())
    
    }

    fn add_file(&mut self, file: PathBuf, prefix: String, connector: String) {
        let new_file = File::new(file, input::Cli::from_args().created_time.to_string(), &self.file_styles);
        let file_name: String = if self.show_file_metadata == "TRUE" {new_file.display_format()} else {new_file.get_name()};
        self.tree.push(format!("{}{} {}", prefix, connector, file_name));
    }
}