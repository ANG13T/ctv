pub struct Directory {
    paths: Vec<File>,
    args: input::Cli
}


impl Directory {
    fn new(args: input::Cli) -> Result<Self, Box<dyn std::error::Error>> {
      let dir = &args.dir;
  
      if !std::path::Path::new(&dir).exists() {
        return Err(
          Box::new(
            std::io::Error::from_raw_os_error(2)
          )
        )
      }
  
      if !std::path::Path::new(&dir).is_dir() {
        let f = File::new(dir.to_owned(), args.time_format, 0);
        match args.long {
          true => print!("{:?}", f),
          _ => print!("{}", f)
        }
        std::process::exit(0)
      }
  
      let paths = std::fs::read_dir(dir)?
          .map(|res| res.map(|e| File::new(
            e.path(), args.time_format.to_owned(), 0
              )
            )
          )
          .collect::<Result<Vec<File>, std::io::Error>>()?;
        Ok(Self { paths, args })
    }
  }