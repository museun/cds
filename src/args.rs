use std::path::PathBuf;

use anyhow::Context;
use chorts::{Features, Target};
use clap::{Arg, ArgAction};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ClassifyKind {
    AssociatedConstant,
    AssociatedFunction,
    AssociatedType,
    Constant,
    Crate,
    Enum,
    Function,
    Macro,
    Method,
    Struct,
    StructField,
    Trait,
    TypeAlias,
    Variant,
    Static,
}

impl ClassifyKind {
    pub const fn as_key(&self) -> &'static str {
        match self {
            Self::AssociatedConstant => "associated_constant",
            Self::AssociatedFunction => "associated_function",
            Self::AssociatedType => "associated_type",
            Self::Constant => "constant",
            Self::Crate => "crate",
            Self::Enum => "enum",
            Self::Function => "function",
            Self::Macro => "macro",
            Self::Method => "method",
            Self::Struct => "struct",
            Self::StructField => "struct_field",
            Self::Trait => "trait",
            Self::TypeAlias => "type_alias",
            Self::Variant => "variant",
            Self::Static => "static",
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::AssociatedConstant => "associated constant",
            Self::AssociatedFunction => "associated function",
            Self::AssociatedType => "associated type",
            Self::Constant => "constant",
            Self::Crate => "the crate",
            Self::Enum => "enum",
            Self::Function => "function",
            Self::Macro => "macro",
            Self::Method => "method",
            Self::Struct => "struct",
            Self::StructField => "struct field",
            Self::Trait => "trait",
            Self::TypeAlias => "type alias",
            Self::Variant => "variant",
            Self::Static => "static",
        }
    }

    pub fn parse(input: &str) -> Option<(Self, usize)> {
        for (k, v) in [
            Self::AssociatedConstant,
            Self::AssociatedFunction,
            Self::AssociatedType,
            Self::Constant,
            Self::Crate,
            Self::Enum,
            Self::Function,
            Self::Macro,
            Self::Method,
            Self::Struct,
            Self::StructField,
            Self::Trait,
            Self::TypeAlias,
            Self::Variant,
            Self::Static,
        ]
        .iter()
        .copied()
        .map(|this| (this.as_str(), this))
        {
            let Some(p) = input.strip_suffix(k) else {
                continue;
            };
            return Some((v, p.len()));
        }
        None
    }
}

impl clap::ValueEnum for ClassifyKind {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::AssociatedConstant,
            Self::AssociatedFunction,
            Self::AssociatedType,
            Self::Constant,
            Self::Crate,
            Self::Enum,
            Self::Function,
            Self::Macro,
            Self::Method,
            Self::Struct,
            Self::StructField,
            Self::Trait,
            Self::TypeAlias,
            Self::Variant,
            Self::Static,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.as_key()))
    }
}

#[derive(Debug)]
pub struct Args {
    pub path: PathBuf,
    pub errors: bool,
    pub panics: bool,
    pub safety: bool,

    pub target: Target,
    pub features: Features,

    pub show_item: bool,
    pub compact: bool,
    pub nightly: bool,

    pub ignore_config: bool,
    pub print_default_config: bool,
    pub print_config_path: bool,

    pub filter: Vec<PathBuf>,
    pub include: Vec<ClassifyKind>,
    pub exclude: Vec<ClassifyKind>,
}

impl Args {
    pub fn parse() -> anyhow::Result<Self> {
        let cmd = clap::Command::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            .about("reports where documentation is missing")
            .arg(
                Arg::new("path")
                    .long("manifest-path")
                    .action(ArgAction::Set)
                    .value_parser(clap::value_parser!(PathBuf))
                    .default_value(".")
                    .help("path to the Cargo.toml you want to use"),
            )
            .arg(
                Arg::new("nightly")
                    .long("nightly")
                    .action(ArgAction::SetTrue)
                    .help("use the nightly version of the toolchain"),
            )
            .arg(
                Arg::new("errors")
                    .long("error")
                    .help_heading("extra warnings")
                    .help("checks for publicly visible functions that return a Result type")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("panics")
                    .long("panic")
                    .help("checks for publicly visible functions that may panic")
                    .help_heading("extra warnings")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("safety")
                    .long("safety")
                    .help("checks for publicly visible unsafe functions")
                    .help_heading("extra warnings")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("all")
                    .short('a')
                    .long("all")
                    .help("enables all extra warnings")
                    .help_heading("extra warnings")
                    .action(ArgAction::SetTrue),
            )
            //
            .arg(
                Arg::new("lib")
                    .long("lib")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "bins",
                        "bin",
                        "examples",
                        "example",
                        "tests",
                        "test",
                        "benches",
                        "bench",
                        "all_targets",
                    ])
                    .action(ArgAction::SetTrue)
                    .help("check only this package's library"),
            )
            .arg(
                Arg::new("bins")
                    .long("bins")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bin",
                        "examples",
                        "example",
                        "tests",
                        "test",
                        "benches",
                        "bench",
                        "all_targets",
                    ])
                    .action(ArgAction::SetTrue)
                    .help("check all binaries"),
            )
            .arg(
                Arg::new("bin")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bins",
                        "examples",
                        "example",
                        "tests",
                        "test",
                        "benches",
                        "bench",
                        "all_targets",
                    ])
                    .long("bin")
                    .help("check only the specified binary")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("examples")
                    .long("examples")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bins",
                        "bin",
                        "example",
                        "tests",
                        "test",
                        "benches",
                        "bench",
                        "all_targets",
                    ])
                    .action(ArgAction::SetTrue)
                    .help("check all examples"),
            )
            .arg(
                Arg::new("example")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bins",
                        "bin",
                        "examples",
                        "tests",
                        "test",
                        "benches",
                        "bench",
                        "all_targets",
                    ])
                    .long("example")
                    .help("check only the specified example")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("tests")
                    .long("tests")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bins",
                        "bin",
                        "examples",
                        "example",
                        "test",
                        "benches",
                        "bench",
                        "all_targets",
                    ])
                    .action(ArgAction::SetTrue)
                    .help("check all targets that have `test = true` set"),
            )
            .arg(
                Arg::new("test")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bins",
                        "bin",
                        "examples",
                        "example",
                        "tests",
                        "benches",
                        "bench",
                        "all_targets",
                    ])
                    .long("test")
                    .help("check only the specified test target")
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("benches")
                    .long("benches")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bins",
                        "bin",
                        "examples",
                        "example",
                        "tests",
                        "test",
                        "bench",
                        "all_targets",
                    ])
                    .action(ArgAction::SetTrue)
                    .help("check all targets that have `bench = true` set"),
            )
            .arg(
                Arg::new("bench")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib",
                        "bins",
                        "bin",
                        "examples",
                        "example",
                        "tests",
                        "test",
                        "benches",
                        "all_targets",
                    ])
                    .long("bench")
                    .help("check only the specified bench target"),
            )
            .arg(
                Arg::new("all_targets")
                    .long("all-targets")
                    .help_heading("targets")
                    .conflicts_with_all([
                        "lib", "bins", "bin", "examples", "example", "tests", "test", "benches",
                        "bench",
                    ])
                    .action(ArgAction::SetTrue)
                    .help("check all targets"),
            )
            //
            .arg(
                Arg::new("features")
                    .short('F')
                    .long("features")
                    .help_heading("features")
                    .conflicts_with_all(["all_features", "no_features"])
                    .help("space or comma separated list of features to activate")
                    .action(ArgAction::Append),
            )
            .arg(
                Arg::new("all_features")
                    .long("all-features")
                    .help_heading("features")
                    .conflicts_with_all(["features", "no_features"])
                    .help("activate all available features")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("no_features")
                    .long("no-default-features")
                    .help_heading("features")
                    .conflicts_with_all(["all_features", "features"])
                    .help("do not activate the `default` feature")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("show_item")
                    .short('s')
                    .long("show-item")
                    .help("show the item this message is attached to")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("compact")
                    .short('c')
                    .long("compact")
                    .help("tries to make things more compact")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("ignore_config")
                    .long("ignore-config")
                    .help_heading("configuration")
                    .help("ignore the on-disk configuration")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("print_default_config")
                    .long("print-default-config")
                    .help_heading("configuration")
                    .help("print the default configuration and exit")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("print_config_path")
                    .long("print-config-path")
                    .help_heading("configuration")
                    .help("print the configuration location and exit")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("filter")
                    .id("glob")
                    .long("filter")
                    .short('f')
                    .action(ArgAction::Append)
                    .group("filtering")
                    .help_heading("filtering")
                    .help("focus these files")
                    .long_help("given a glob, only shows the files that match it."),
            )
            .arg(
                Arg::new("include")
                    .short('i')
                    .long("include")
                    .help("include only specific lint kinds")
                    .conflicts_with("exclude")
                    .group("filtering")
                    .help_heading("filtering")
                    .value_parser(clap::value_parser!(ClassifyKind))
                    .action(ArgAction::Append),
            )
            .arg(
                Arg::new("exclude")
                    .short('e')
                    .long("exclude")
                    .help("exclude specific lint kinds")
                    .group("filtering")
                    .help_heading("filtering")
                    .conflicts_with("include")
                    .value_parser(clap::value_parser!(ClassifyKind))
                    .action(ArgAction::Append),
            );

        let mut matches = cmd.get_matches();

        let path = matches
            .remove_one("path")
            .unwrap_or_else(|| PathBuf::from("."));
        let path = chorts::locate_manifest(path)?;

        let mut this = Self {
            errors: matches.get_flag("errors"),
            panics: matches.get_flag("panics"),
            safety: matches.get_flag("safety"),
            target: Target::parse(&mut matches),
            features: Features::parse(&mut matches),
            nightly: matches.get_flag("nightly"),
            compact: matches.get_flag("compact"),

            show_item: matches.get_flag("show_item"),

            ignore_config: matches.get_flag("ignore_config"),
            print_config_path: matches.get_flag("print_config_path"),
            print_default_config: matches.get_flag("print_default_config"),

            include: matches
                .remove_many("include")
                .into_iter()
                .flatten()
                .collect(),

            exclude: matches
                .remove_many("exclude")
                .into_iter()
                .flatten()
                .collect(),

            filter: glob_filters(
                {
                    let mut parent = path.clone();
                    parent.pop();
                    parent
                },
                matches.remove_many::<String>("glob").into_iter().flatten(),
            )?,
            path,
        };

        if matches.get_flag("all") {
            this.errors = true;
            this.panics = true;
            this.safety = true;
        }

        Ok(this)
    }
}

fn glob_filters(
    root: PathBuf,
    patterns: impl IntoIterator<Item = String>,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths = vec![];

    for pattern in patterns {
        let new = root.join(pattern);
        for pat in glob::glob(&new.to_string_lossy())
            .with_context(|| anyhow::anyhow!("invalid glob: {}", new.display()))?
        {
            let pat = pat?;
            let local = pat.strip_prefix(&root).unwrap_or(&pat);
            if local.starts_with("target") {
                continue;
            }
            paths.push(local.to_path_buf());
        }
    }

    Ok(paths)
}
