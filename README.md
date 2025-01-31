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
