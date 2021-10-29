use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    // parses args into our CLI struct
    // Clap knows which fields to expect, and what their expected format is
    // Automatic error catching
    let args = Cli::from_args();
    println!("Hello, world!");
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("Hello, world!");
}
