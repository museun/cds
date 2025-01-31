use std::collections::HashSet;

use chorts::Visit as _;

mod args;
use args::Args;

mod config;
use config::Config;
use render::Options;

mod render;
mod visit;

fn main() -> anyhow::Result<()> {
    let args = Args::parse()?;

    let path = Config::initial_config(args.ignore_config)?;

    if args.print_config_path {
        println!("{}", path.display());
        std::process::exit(0)
    }

    if args.print_default_config {
        println!("{}", Config::DEFAULT);
        std::process::exit(0)
    }

    let config = if args.ignore_config {
        Config::default()
    } else {
        Config::load(path)?
    };

    let path = chorts::locate_manifest(&args.path)?;

    let mut cmd = chorts::Command::default()
        .with_tool(chorts::Tool::Clippy)
        .with_flags([
            chorts::Flag::new("-W", "missing_docs"),
            chorts::Flag::new("-W", "clippy::empty_docs"),
            chorts::Flag::new("-W", "clippy::suspicious_doc_comments"),
        ])
        .with_target(args.target)
        .with_features(args.features)
        .with_manifest_path(path)?;

    for (extra, flag) in [
        (args.errors, "clippy::missing-errors-doc"),
        (args.panics, "clippy::missing-panics-doc"),
        (args.safety, "clippy::missing-safety-doc"),
        (args.safety, "clippy::unnecessary_safety_doc"),
        (args.safety, "clippy::undocumented_unsafe_blocks"),
    ] {
        if extra {
            cmd = cmd.with_flag(chorts::Flag::new("-W", flag))
        }
    }

    if args.nightly {
        cmd = cmd.with_toolchain(chorts::Toolchain::Nightly);
    }

    let reasons = cmd.gather()?;

    let set = args.filter.iter().collect::<HashSet<_>>();
    let mut docs = visit::MissingDocs::new(set, args.include, args.exclude);
    reasons.accept(&mut docs);

    let options = Options {
        compact: args.compact,
        show_item: args.show_item,
    };

    render::show(docs, options, config);

    Ok(())
}
