pub fn extension(file: &std::path::Path) -> anyhow::Result<String> {
    if file.is_dir() {
        return Ok("".to_string());
    }
    Ok(file
        .extension()
        .ok_or_else(|| anyhow::anyhow!("File has no extension"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("File extension is invalid UTF-8"))?
        .to_string()
        .to_uppercase())
}
