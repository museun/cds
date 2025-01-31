use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Context as _;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        toml::from_str(Self::DEFAULT).unwrap()
    }
}

impl Config {
    pub const DEFAULT: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "default.config.toml"
    ));

    pub const QUALIFIER: &str = "com.github";
    pub const ORGANIZATION: &str = "museun";
    pub const APPLICATION: &str = env!("CARGO_PKG_NAME");
    pub const FILE_NAME: &str = "config.toml";

    pub fn initial_config(ignore: bool) -> anyhow::Result<PathBuf> {
        let path = Config::get_config_path()
            .with_context(|| anyhow::anyhow!("cannot find XDG_CONFIG_HOME"))?;

        if !path.is_file() && !ignore {
            let yellow = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Yellow.into()));
            let cyan = anstyle::Style::new().fg_color(Some(anstyle::AnsiColor::Cyan.into()));
            let reset = anstyle::Reset;
            anstream::eprintln!(
                "{yellow}WARNING{reset}: creating the default configuration at:\n\t{path}",
                path = path.display()
            );
            anstream::eprintln!("{cyan}NOTE{reset}: you may want to review this file");
            {
                let mut parent = path.clone();
                parent.pop();
                let _ = std::fs::create_dir_all(&parent);
            }
            std::fs::write(path, Config::DEFAULT)?;
            std::process::exit(0)
        }

        Ok(path)
    }

    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let data = std::fs::read_to_string(path).with_context(|| {
            anyhow::anyhow!(
                "cannot read configuration file at {path}",
                path = path.display()
            )
        })?;

        toml::from_str(&data).with_context(|| {
            anyhow::anyhow!(
                "cannot parse configuration file at {path} ",
                path = path.display()
            )
        })
    }

    pub fn get_config_path() -> Option<PathBuf> {
        directories::ProjectDirs::from(
            Self::QUALIFIER, //
            Self::ORGANIZATION,
            Self::APPLICATION,
        )
        .map(|s| s.config_dir().join(Self::FILE_NAME))
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Theme {
    pub file_header: Option<Style>,
    pub file_name: Option<Style>,
    pub location: Option<Style>,
    pub message: Option<Style>,
    pub highlight_code: Option<Style>,
    pub code: Option<Style>,
    #[serde(default)]
    pub kinds: Kinds,
}

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Kinds {
    #[serde(rename = "associated_constant")]
    pub associated_constant: Option<Style>,
    #[serde(rename = "associated_function")]
    pub associated_function: Option<Style>,
    #[serde(rename = "enum")]
    pub enumeration: Option<Style>,
    #[serde(rename = "function")]
    pub function: Option<Style>,
    #[serde(rename = "method")]
    pub method: Option<Style>,
    #[serde(rename = "module")]
    pub module: Option<Style>,
    #[serde(rename = "struct")]
    pub structure: Option<Style>,
    #[serde(rename = "struct_field")]
    pub struct_field: Option<Style>,
    #[serde(rename = "trait")]
    pub traity: Option<Style>,
    #[serde(rename = "variant")]
    pub variant: Option<Style>,
    #[serde(rename = "the_crate")]
    pub the_crate: Option<Style>,
}

impl From<Style> for anstyle::Style {
    fn from(value: Style) -> Self {
        let mut this = {
            if let Some(Color(r, g, b)) = value.color {
                Self::new().fg_color(Some(anstyle::Color::Rgb(anstyle::RgbColor(r, g, b))))
            } else {
                Self::new()
            }
        };
        type Apply = fn(anstyle::Style) -> anstyle::Style;
        for (val, apply) in [
            (value.bold, anstyle::Style::bold as Apply),
            (value.italic, anstyle::Style::italic),
            (value.underline, anstyle::Style::underline),
            (value.dimmed, anstyle::Style::dimmed),
        ] {
            if val {
                this = apply(this)
            }
        }
        this
    }
}

#[derive(Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Style {
    #[serde(default)]
    pub color: Option<Color>,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub italic: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub dimmed: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const fn from_u16(color: u16) -> Self {
        let offset = if ((color >> 12) & ((1 << 4) - 1)) == 0 {
            4
        } else {
            0
        };

        let r = (color >> (12 - offset) & 0xF) as u8;
        let g = (color >> (8 - offset) & 0xF) as u8;
        let b = (color >> (4 - offset) & 0xF) as u8;

        Self((r << 4) | r, (g << 4) | g, (b << 4) | g)
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error as _;
        <String>::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}

impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(r, g, b) = self;
        write!(f, "#{r:02x}{g:02x}{b:02x}")
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(input) = input.strip_prefix('#') {
            return match input.len() {
                3 | 4 => u16::from_str_radix(input, 16)
                    .map_err(|s| s.to_string())
                    .map(Self::from_u16),
                6 | 8 => u32::from_str_radix(input, 16)
                    .map_err(|s| s.to_string())
                    .map(|num| {
                        let [_, r, g, b] = num.to_be_bytes();
                        Self(r, g, b)
                    }),
                _ => {
                    return Err(String::from(
                        "invalid syntax for color. rgb(r,g,b) or #RRGGBB or #RGB",
                    ))
                }
            };
        }

        if input.starts_with("rgb(") && input.ends_with(")") {
            let input = &input[4..input.len() - 1];
            let mut iter = input.split_terminator(',').map(|s| s.trim().parse());

            let r = iter
                .next()
                .and_then(Result::ok)
                .ok_or_else(|| String::from("invalid red channel"))?;
            let b = iter
                .next()
                .and_then(Result::ok)
                .ok_or_else(|| String::from("invalid blue channel"))?;
            let g = iter
                .next()
                .and_then(Result::ok)
                .ok_or_else(|| String::from("invalid green channel"))?;
            return Ok(Self(r, g, b));
        }

        Err(String::from("invalid color"))
    }
}
