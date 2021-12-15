extern crate dotenv;
use dotenv::dotenv;
use std::{fs, env};
use structopt::StructOpt;
use std::path::{PathBuf};
use std::error::Error;
use crate::protocols;
use crate::input;

#[derive(Clone)]
pub struct TreeGenerator {
    root_dir: PathBuf,
    tree: Vec<String>,
    PIPE: String,
    ELBOW: String, 
    TEE: String,
    PIPE_PREFIX: String,
    SPACE_PREFIX: String
}

impl TreeGenerator {
    fn init(&mut self, root_dir: PathBuf) {
        dotenv().ok();
        self.tree = Vec::new();
        self.PIPE = env::var("PIPE").unwrap();
        self.ELBOW = env::var("ELBOW").unwrap();
        self.TEE = env::var("TEE").unwrap();
        self.PIPE_PREFIX = env::var("PIPE_PREFIX").unwrap();
        self.SPACE_PREFIX = env::var("SPACE_PREFIX").unwrap();
    }
    fn build_tree(&mut self) -> Vec<String>{
        self.tree_head();
        self.tree_body(self.root_dir.clone(), "".to_string());
        return self.tree.clone();
    }

    fn sort_dir_first(&self, directory: PathBuf) -> Result<Vec<fs::DirEntry>, Box<dyn Error>>{
        let mut dirVec: Vec<fs::DirEntry> = Vec::new();
        let mut fileVec: Vec<fs::DirEntry> = Vec::new(); 
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
    
            let metadata = fs::metadata(&path)?;
    
            if metadata.is_file(){
                fileVec.push(entry);
            }else if metadata.is_dir(){
                dirVec.push(entry);
            }
        }

        dirVec.append(&mut fileVec);
        Ok(dirVec)
    }

    fn tree_head(&mut self) {
        let dirFile = protocols::File::new(self.root_dir.clone(), input::Cli::from_args().created_time.to_string());
        self.tree.push(dirFile.displayFormat());
        self.tree.push(self.PIPE.clone());
    }

    fn tree_body(&self, directory: PathBuf, prefix: String) {

        let entries = self.sort_dir_first(directory).unwrap();
        let entries_count = entries.len();
        

        for (index, entry) in entries.iter().enumerate(){
            let mut connector;
            if index == entries_count - 1 {
                connector = &self.ELBOW;
            }else{
                connector = &self.TEE;
            }

            let metadata = fs::metadata(entry.path()).unwrap();

            if metadata.is_dir() {
                self.add_directory(
                    entry.path(), entry.path(), index, entries_count, prefix, prefix, connector.to_string()
                )
            }else {
                self.add_file(entry.path(), prefix, connector.to_string())
            }
        }
                
    }

    fn add_directory(&mut self, directory: PathBuf, directory2: PathBuf, index: usize, entries_count: usize, mut prefix: String, prefix2: String, connector: String) {
        let newFile = protocols::File::new(directory, input::Cli::from_args().created_time.to_string());
        let fileName = newFile.getName();
        self.tree.push(format!("{}{} {}", prefix, connector, fileName));
        if index != entries_count - 1 {
            prefix += &self.PIPE_PREFIX;
        }else {
            prefix += &self.SPACE_PREFIX;
        }
            
        self.tree_body(
            directory2,
            prefix
        );
        self.tree.push(prefix2.trim_end().to_string());
    }

    fn add_file(&mut self, file: PathBuf, prefix: String, connector: String) {
        let newFile = protocols::File::new(file, input::Cli::from_args().created_time.to_string());
        let fileName: String = newFile.getName();
        self.tree.push(format!("{}{} {}", prefix, connector, fileName));
    }
}