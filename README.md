# Cargo Toml Formatting Tool

This is an in-progress, non-production-ready, comments-preserving, cargo TOML file formatting library.
See the lists below for detailed information on what formatting is supported. 

## How to use

You can use this tool, its not a cargo published binary yet, see `main.rs` or the tests as examples on how to use this library from code. There are test for all cases that are guaranteed to work. However, sorting/ordering/formatting does require a certain order in order to work properly. Adviced is to do your toml formatting in phases rather then all features at once. 

## Definitions

First, the definitions used in this document.

- A section is defined by `[some_name]` and contains key-value pairs.
- A key identifies a value.
- A value can be an table `{...}`, array `[]`, or inlined datatype `key = value`.
- A table contains 1 or many key-value pairs.
- A table can be defined as inline or not inline.

## Formatting Rules

Formatting rules are defined by the rust FMT RFC [^1] and cargo manifest [^2]. However, there maybe rules here that do not conform or are not yet stabalized as 'official'.

### Sections

- [X] `package` section should always be at the top.
- [X] `package` section order as it is defined in the manifest is maintained.
- [X] Section header and the first following keys should NOT be separated by space.
- [X] Sections are separated by a single newline.

### Comments

- [x] If the comment is inline, on the same line as an item, it should be separated from the item by one space.
- [x] Arrays where each line is wrapped on a new line may contain comments at the line ending to elaborate on certain array items.
- [ ] Arrays where each line is wrapped on a new line may contain comments as entries to elaborate on certain arrray items.


## Keys

- [X] Keys within sections have no spaces in between them except for the usecase of grouping.
- [x] Groups in sections are defined by a single white space. 
- [X] Keys may not contain quotes.
- [x] Keys are separated from a value by ` = `
- [x] Keys may not contain quotes (unless it is required for a particular reason)
- [x] Keys are sorted alphabetically within each section, except for the [package] section.
- [x] Empty spaces are stripped at the start and end of each line.

## Line Length Wrap

- [X] Arrays wrap line when longer than configurable length.
- [X] For table values, such as a crate dependency with a path, write the entire table using curly braces and commas on the same line as the key if it fits. If the entire table does not fit on the same line as the key, separate it out into a separate section with key-value pairs.
- [ ] Within the description field, wrap text at 80 columns
- [ ] Use multiline strings rather than `\n`.

## Field Restrictions

- [ ] The `license` field, if present, must contain a valid SPDX expression, using valid SPDX license names. (As an exception, by widespread convention, the license field may use / in place of OR; for example, MIT/Apache-2.0.) [^6]
- [ ] The `homepage`, `documentation`, and `repository` field, if present, must consist of a single URL, including the scheme (e.g. https://example.org/, not just example.org.)

- [ ] The `name` field must use only alphanumeric characters or - or _, and cannot be empty. Note that cargo new and cargo init impose some additional restrictions on the package name, such as enforcing that it is a valid Rust identifier and not a keyword. crates.io imposes even more restrictions, such as enforcing only ASCII characters, not a reserved name, not a special Windows name such as "nul", is not too long, etc. [^3]
- [ ] The `edition`, if present, field may only contain one of the following: `2015`, `2018`, and `2021` [^4]
- [ ] The `rust-version`, if present, must be a bare version number with two or three components; it cannot include semver operators or pre-release identifiers. [^5]
- [ ] The `readme`, and `build`, field, if present, must be an existing file.
- [ ] The `keywords` field its keyword must be ASCII text, start with a letter, and only contain letters, numbers, _ or -, and have at most 20 characters. [^7]
- [ ] The `categories` field its categories, if present, should match one of the strings available at https://crates.io/category_slugs, and must match exactly.
- [ ] The `publish` field, if present, must be a boolean or array with registry links. [^8]

# Similar Work

While there is similar work it was not what I was looking for.

- [cargo-toml-lint](https://crates.io/crates/cargo-toml-lint)
    Lints if dependency sections are sorted alphabetically.
- [Rust fmt PR](https://github.com/rust-lang/rustfmt/pull/5240/files)
    Provides basic formatting operations but has been open for a long time and is not a high priority. This is the reason this library is created.
- [cargo-toml-fmt](https://github.com/tbrand/cargo-tomlfmt)
    Very minimal formatting library that is not maintained for many years.
- [cargo-sort](https://github.com/DevinR528/cargo-sort)
    Sorts tables in the cargo toml file but lacks.
- [toml-fmt](https://crates.io/crates/toml-fmt)
    Limited non-maintained toml formatting library.


## TODO

- More control over excluding items from the formatting processes.
- Add binary functionality with file configuration in yml format.
- Create linter.

- Add grouping support for ordering dependencies.
- section key trimmer removes space from key assignment
- section quote trimmer removes comments
- array formatting removes comments.



Needs validation
- table formatting section trim?
- validate section comments do not get stripped
- 

[^1]: https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/cargo.md?rgh-link-date=2020-04-11T05%3A30%3A22Z
[^2]: https://doc.rust-lang.org/cargo/reference/manifest.html
[^3]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field
[^4]: https://doc.rust-lang.org/edition-guide/index.html
[^5]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field
[^6]: https://spdx.github.io/license-list-data/
[^7]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field
[^8]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field


