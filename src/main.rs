use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// return types of function can either be error or default return value
// Box<dyn std::error::Error>> => It’s a Box that can contain any type that implements the standard Error trait. 
// This means that basically all errors can be put into this box, so we can use ? on all of the usual functions that return Results.
fn main() -> Result<(), Box<dyn std::error::Error>>{
    // read read file from string
    // question mark at end automatically returns an error if path is incorrect
    let result = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}
