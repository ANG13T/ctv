extern crate dotenv;
use dotenv::dotenv;
use std::{fs, env};
use structopt::StructOpt;
use std::path::{PathBuf};
use std::error::Error;
use crate::protocols;
use walkdir::WalkDir;
use crate::input;

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
    fn build_tree(&self) -> Vec<String>{
        self.tree_head();
        self.tree_body(self.root_dir.clone(), "".to_string());
        return self.tree.clone();
    }
    fn sort_dir_first(directory: PathBuf) -> Result<Vec<fs::DirEntry>, Box<dyn Error>>{
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
        Ok((dirVec))
    }

    fn tree_head(&self) {
        let dirFile = protocols::File::new(self.root_dir, input::Cli::from_args().created_time.to_string());
        self.tree.push(dirFile.displayFormat());
        self.tree.push(self.PIPE);
    }
    fn tree_body(&self, directory: PathBuf, prefix: String) {

        let entries = TreeGenerator::sort_dir_first(directory);
        

        // for index, entry in enumerate(entries):
        //     connector = ELBOW if index == entries_count - 1 else TEE;

        //     if entry.is_dir() {
        //         self._add_directory(
        //             entry, index, entries_count, prefix, connector
        //         )
        //     }else {
        //         self._add_file(entry, prefix, connector)
        //     }
                
    }

    fn add_directory(&self, directory: PathBuf, index: i32, entries_count: i32, prefix: String, connector: String) {
        let newFile = protocols::File::new(directory, input::Cli::from_args().created_time.to_string());
        let fileName = newFile.getName();
        self.tree.push(format!("{}{} {}", prefix, connector, fileName));
        if index != entries_count - 1 {
            prefix += &self.PIPE_PREFIX;
        }else {
            prefix += &self.SPACE_PREFIX;
        }
            
        self.tree_body(
            directory,
            prefix
        );
        self.tree.push(prefix.trim_right().to_string());
    }

    fn add_file(&self, file: protocols::File, prefix: String, connector: String) {
        let fileName: String = file.getName();
        self.tree.push(format!("{}{} {}", prefix, connector, fileName));
    }
}