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

    if args.search.len() > 0 {
        println!("searching {}", args.search);
    }

    if args.print_config {
        println!("{:#?}\n{:#?}", config, args);
        return Ok(());
    }

    protocols::DirTree::init(&args.dir, config).gen()
}
