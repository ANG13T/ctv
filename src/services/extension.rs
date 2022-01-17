pub fn extension(file: &std::path::PathBuf) -> String{
    if file.is_dir() {return "".to_string();}
    return file.extension().unwrap().to_str().unwrap().to_string().to_uppercase();
}