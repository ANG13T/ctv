mod args;
mod config;
mod protocols;
mod services;

fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    let mut config = config::load().context("Reading configuration")?;
    let args = args::parse().context("Parsing arguments")?;

    if args.short {
        config.view_format = config::ViewFormat::Short;
    }
    if let Some(limit) = args.limit {
        config.max_depth = limit;
    }

    if args.print_config {
        println!("{:#?}\n{:#?}", config, args);
        return Ok(());
    }

    protocols::DirTree::new(&args.dir.canonicalize()?, &config)?.write(&mut std::io::stdout())
}
