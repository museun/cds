## reports where documentation is missing

```
Usage: cds [OPTIONS]

Options:
      --manifest-path <path>
          path to the Cargo.toml you want to use

          [default: .]

      --nightly
          use the nightly version of the toolchain

  -s, --show-item
          show the item this message is attached to

  -c, --compact
          tries to make things more compact

  -f, --filter <glob>
          given a glob, only shows the files that match it.

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

extra warnings:
      --error
          checks for publicly visible functions that return a Result type

      --panic
          checks for publicly visible functions that may panic

      --safety
          checks for publicly visible unsafe functions

  -a, --all
          enables all extra warnings

targets:
      --lib
          check only this package's library

      --bins
          check all binaries

      --bin <bin>
          check only the specified binary

      --examples
          check all examples

      --example <example>
          check only the specified example

      --tests
          check all targets that have `test = true` set

      --test <test>
          check only the specified test target

      --benches
          check all targets that have `bench = true` set

      --bench <bench>
          check only the specified bench target

      --all-targets
          check all targets

features:
  -F, --features <features>
          space or comma separated list of features to activate

      --all-features
          activate all available features

      --no-default-features
          do not activate the `default` feature

configuration:
      --ignore-config
          ignore the on-disk configuration

      --print-default-config
          print the default configuration and exit

      --print-config-path
          print the configuration location and exit

```

## configuration

```toml
# style syntax
#   color: rgb(base10, base10, base10)
#   color: #RGB (base 16)
#   color: #RRGGBB (base 16)
#   bold: bool
#   italic: bool
#   underline: bool
#   dimmed: bool


# the style of the filename in the first-line
[theme.file_header]
color = "#56b6c2"
bold = true
italic = true

# the style of the filename in each result
[theme.file_name]
color = "#AAAAAA"

# the style of the row/col in each result
[theme.location]
dimmed = true

# the style of the highlighted code
[theme.highlight_code]
color = "#e8a"
italic = true

# the style of the normal code
[theme.code]

# the style of the missing docs message
[theme.message]
color = "#999999"

# styles for matching specific 'kinds' of items
# [theme.kinds.associated_constant]

# [theme.kinds.associated_function]

# [theme.kinds.enum]

# [theme.kinds.function]

# [theme.kinds.method]

# [theme.kinds.module]

# [theme.kinds.struct]
# color = "#FF00FF"

# [theme.kinds.struct_field]

# [theme.kinds.trait]

# [theme.kinds.variant]

# [theme.kinds.the_crate]
```
