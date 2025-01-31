## reports where documentation is missing

```
reports where documentation is missing

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

filtering:
  -f, --filter <glob>
          given a glob, only shows the files that match it.

  -i, --include <include>
          include only specific lint kinds

          [possible values: associated_constant, associated_function,
          associated_type, constant, crate, enum, function, macro, method,
          struct, struct_field, trait, type_alias, variant, static]

  -e, --exclude <exclude>
          exclude specific lint kinds

          [possible values: associated_constant, associated_function,
          associated_type, constant, crate, enum, function, macro, method,
          struct, struct_field, trait, type_alias, variant, static]
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

# the style of the missing docs message
[theme.message]
color = "#999999"

# the style of the highlighted code
[theme.highlight_code]
color = "#e8a"
italic = true

# the style of the normal code
# [theme.code]

# styles for matching specific 'kinds' of items
# [theme.kinds.associated_constant]
# [theme.kinds.associated_function]
# [theme.kinds.associated_type]
# [theme.kinds.constant]
# [theme.kinds.crate]
# [theme.kinds.enum]
# [theme.kinds.function]
# [theme.kinds.macro]
# [theme.kinds.method]
# [theme.kinds.struct]
# [theme.kinds.struct_field]
# [theme.kinds.trait]
# [theme.kinds.type_alias]
# [theme.kinds.variant]
# [theme.kinds.static]

```

## examples

reference project:

```rust
pub static FOO: () = ();
pub const BAR: () = ();

pub type Baz = ();
pub struct Quux {
    pub field: (),
}

pub use std::mem::drop as drop_it;

impl Quux {
    pub const SOMETHING: () = ();
    pub fn foo() {}
    pub fn bar(&self) {}
    pub fn baz(_this: &Self) {}
}

pub trait Foo {
    type Baz;
    fn bar(&self);
    fn baz(&self) {}
}

impl Foo for () {
    type Baz = Asdf;
    fn bar(&self) {}
}

pub fn foo() {}

pub enum Asdf {}
pub enum Fdsa {
    Foo {
        d: i32,
    },
    Bar,
    Baz(
        //
        i32,
    ),
}

#[macro_export]
macro_rules! asdf {
    () => {};
}
```

> cds

```
in src/lib.rs
  src/lib.rs:1:1   missing documentation for the crate
  src/lib.rs:1:1   missing documentation for a static
  src/lib.rs:2:1   missing documentation for a constant
  src/lib.rs:4:1   missing documentation for a type alias
  src/lib.rs:5:1   missing documentation for a struct
  src/lib.rs:6:5   missing documentation for a struct field
  src/lib.rs:12:5  missing documentation for an associated constant
  src/lib.rs:13:5  missing documentation for an associated function
  src/lib.rs:14:5  missing documentation for a method
  src/lib.rs:15:5  missing documentation for an associated function
  src/lib.rs:18:1  missing documentation for a trait
  src/lib.rs:19:5  missing documentation for an associated type
  src/lib.rs:20:5  missing documentation for a method
  src/lib.rs:21:5  missing documentation for a method
  src/lib.rs:29:1  missing documentation for a function
  src/lib.rs:31:1  missing documentation for an enum
  src/lib.rs:32:1  missing documentation for an enum
  src/lib.rs:33:5  missing documentation for a variant
  src/lib.rs:34:9  missing documentation for a struct field
  src/lib.rs:36:5  missing documentation for a variant
  src/lib.rs:37:5  missing documentation for a variant
  src/lib.rs:44:1  missing documentation for a macro
```

> cds --compact

```
in src/lib.rs
  src/lib.rs:1:1   missing documentation for the crate
  src/lib.rs:1:1   static
  src/lib.rs:2:1   constant
  src/lib.rs:4:1   type alias
  src/lib.rs:5:1   struct
  src/lib.rs:6:5   struct field
  src/lib.rs:12:5  associated constant
  src/lib.rs:13:5  associated function
  src/lib.rs:14:5  method
  src/lib.rs:15:5  associated function
  src/lib.rs:18:1  trait
  src/lib.rs:19:5  associated type
  src/lib.rs:20:5  method
  src/lib.rs:21:5  method
  src/lib.rs:29:1  function
  src/lib.rs:31:1  enum
  src/lib.rs:32:1  enum
  src/lib.rs:33:5  variant
  src/lib.rs:34:9  struct field
  src/lib.rs:36:5  variant
  src/lib.rs:37:5  variant
  src/lib.rs:44:1  macro
```

> cds --compact --show-item

```
in src/lib.rs
  src/lib.rs:1:1   missing documentation for the crate
  src/lib.rs:1:1   static
    pub static FOO: () = ();
  src/lib.rs:2:1   constant
    pub const BAR: () = ();
  src/lib.rs:4:1   type alias
    pub type Baz = ();
  src/lib.rs:5:1   struct
    pub struct Quux {
  src/lib.rs:6:5   struct field
    pub field: (),
  src/lib.rs:12:5  associated constant
    pub const SOMETHING: () = ();
  src/lib.rs:13:5  associated function
    pub fn foo() {}
  src/lib.rs:14:5  method
    pub fn bar(&self) {}
  src/lib.rs:15:5  associated function
    pub fn baz(_this: &Self) {}
  src/lib.rs:18:1  trait
    pub trait Foo {
  src/lib.rs:19:5  associated type
    type Baz;
  src/lib.rs:20:5  method
    fn bar(&self);
  src/lib.rs:21:5  method
    fn baz(&self) {}
  src/lib.rs:29:1  function
    pub fn foo() {}
  src/lib.rs:31:1  enum
    pub enum Asdf {}
  src/lib.rs:32:1  enum
    pub enum Fdsa {
  src/lib.rs:33:5  variant
    Foo {
  src/lib.rs:34:9  struct field
    d: i32,
  src/lib.rs:36:5  variant
    Bar,
  src/lib.rs:37:5  variant
    Baz(
  src/lib.rs:44:1  macro
    macro_rules! asdf {
```

> cds --compact --show-item --include method

```
in src/lib.rs
  src/lib.rs:14:5  method
    pub fn bar(&self) {}
  src/lib.rs:20:5  method
    fn bar(&self);
  src/lib.rs:21:5  method
    fn baz(&self) {}
```

> cds --compact --show-item --exclude struct --exclude struct_field --exclude variant --exclude crate

```
in src/lib.rs
  src/lib.rs:1:1   static
    pub static FOO: () = ();
  src/lib.rs:2:1   constant
    pub const BAR: () = ();
  src/lib.rs:4:1   type alias
    pub type Baz = ();
  src/lib.rs:12:5  associated constant
    pub const SOMETHING: () = ();
  src/lib.rs:13:5  associated function
    pub fn foo() {}
  src/lib.rs:14:5  method
    pub fn bar(&self) {}
  src/lib.rs:15:5  associated function
    pub fn baz(_this: &Self) {}
  src/lib.rs:18:1  trait
    pub trait Foo {
  src/lib.rs:19:5  associated type
    type Baz;
  src/lib.rs:20:5  method
    fn bar(&self);
  src/lib.rs:21:5  method
    fn baz(&self) {}
  src/lib.rs:29:1  function
    pub fn foo() {}
  src/lib.rs:31:1  enum
    pub enum Asdf {}
  src/lib.rs:32:1  enum
    pub enum Fdsa {
  src/lib.rs:44:1  macro
    macro_rules! asdf {
```
