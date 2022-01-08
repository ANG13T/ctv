pub fn extension(file: &std::path::PathBuf) -> String{
    return file.extension().unwrap().to_str().unwrap().to_string();
}