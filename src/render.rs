use chorts::Highlight;

use crate::{
    args::ClassifyKind,
    config::{Config, Style, Theme},
    visit::MissingDocs,
};

pub struct Options {
    pub compact: bool,
    pub show_item: bool,
}

pub fn show(docs: MissingDocs, options: Options, config: Config) {
    let (location, file_name, file_header, message, highlight, code) = (
        theme_style(config.theme.location),
        theme_style(config.theme.file_name),
        theme_style(config.theme.file_header),
        theme_style(config.theme.message),
        theme_style(config.theme.highlight_code),
        theme_style(config.theme.code),
    );

    let reset = anstyle::Reset;

    let padding = pad_locations(&docs);

    for (i, (file, messages)) in docs.map.into_iter().enumerate() {
        if i > 0 {
            anstream::println!("  {sp}", sp = " ".repeat(padding + 1))
        }

        let file = file.to_string_lossy();
        anstream::println!("in {file_header}{file}{reset}",);

        for missing in messages {
            let location = format!(
                "{file_name}{file}{reset}:{location}{row}:{col}{reset}",
                row = missing.message.row,
                col = missing.message.col
            );

            let msg = if options.compact {
                shorten(&missing.message.item)
            } else {
                &missing.message.item
            };

            let sp = " ".repeat(padding.saturating_sub(
                file.len()
                    + 2
                    + count_digits(missing.message.row)
                    + count_digits(missing.message.col),
            ));

            match Classify::classify(msg, &config.theme) {
                Some((head, tail, style)) => {
                    let style = theme_style(Some(style));
                    anstream::println!(
                        "  {location} {sp} {reset}{message}{head}{reset}{style}{tail}{reset}"
                    )
                }
                None => {
                    anstream::println!("  {location} {sp} {message}{msg}{reset}")
                }
            }

            // this string check is because they attach spans to the whole crate
            if options.show_item && !msg.ends_with("the crate") {
                for (head, middle, tail) in partition(&missing.text) {
                    anstream::println!(
                        "    {code}{head}{reset}{highlight}{middle}{reset}{code}{tail}{reset}"
                    )
                }
            }
        }
    }
}

const fn count_digits(d: usize) -> usize {
    let (mut len, mut n) = (1, 1);
    while len < 20 {
        n *= 10;
        if n > d {
            return len;
        }
        len += 1;
    }
    len
}

fn pad_locations(docs: &MissingDocs) -> usize {
    docs.map
        .iter()
        .map(|(k, v)| {
            let left = k.to_str().unwrap().len() + 2;
            let right = v
                .iter()
                .map(|s| count_digits(s.message.row) + count_digits(s.message.col))
                .max()
                .unwrap_or(1);
            left + right
        })
        .max()
        .unwrap_or(1)
}

fn theme_style(style: Option<Style>) -> anstyle::Style {
    style
        .map(anstyle::Style::from)
        .unwrap_or(anstyle::Style::new())
}

fn shorten(input: &str) -> &str {
    let mut input = input;
    for prefix in [
        "missing documentation for a ",
        "missing documentation for an ",
        "docs for function returning `Result` ",
        "docs for function which may panic ",
        "safe function's docs have ",
        "unsafe function's docs are ",
    ] {
        input = input.strip_prefix(prefix).unwrap_or(input)
    }

    input
}

fn floor_char_boundary(str: &str, index: usize) -> usize {
    if index >= str.len() {
        return str.len();
    }

    let start = index.saturating_sub(3);
    let next = str.as_bytes()[start..=index]
        .iter()
        .rposition(|&b| (b as i8) >= -0x40)
        .unwrap();
    start + next
}

fn ceil_char_boundary(str: &str, index: usize) -> usize {
    if index > str.len() {
        return str.len();
    }

    let end = (index + 4).min(str.len());
    str.as_bytes()[index..end]
        .iter()
        .position(|&b| (b as i8) >= -0x40)
        .map_or(end, |pos| pos + index)
}

struct Classify;

impl Classify {
    fn classify<'a>(input: &'a str, theme: &Theme) -> Option<(&'a str, &'a str, Style)> {
        let (kind, offset) = ClassifyKind::parse(input)?;
        let key = kind.as_key();
        let style = theme.kinds.get(key)?;
        let (head, tail) = input.split_at(offset);
        Some((head, tail, *style))
    }
}

fn partition<'a, 'b>(
    spans: &'b [Highlight<'a>],
) -> impl Iterator<Item = (&'a str, &'a str, &'a str)> + use<'a, 'b>
where
    'b: 'a,
{
    let mut iter = spans.iter().enumerate();
    let mut left_pad = 0;

    std::iter::from_fn(move || loop {
        let (i, span) = iter.next()?;
        if span.data.trim_start().is_empty() {
            continue;
        }

        if i == 0 {
            let s = span.data.trim_start();
            left_pad = span.data.len() - s.len();
        }

        let start = span.start.saturating_sub(left_pad + 1);
        let end = span.end.saturating_sub(left_pad + 1);

        let start = str_indices::chars::from_byte_idx(&span.data, start);
        let end = str_indices::chars::from_byte_idx(&span.data, end);

        let text = &span.data[left_pad..];
        let start = floor_char_boundary(text, start);
        let end = ceil_char_boundary(text, end);

        let head = &text[..start];
        let middle = &text[start..end];
        let tail = &text[end..];

        break Some((head, middle, tail));
    })
}
