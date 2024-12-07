# Rust Structure
* Access the root of the module tree using `crate`
* A module is a container for zero or more items.

## Rust Project structure best practice
* It is not allowed to have both `garden.rs` file and `garden/mod.rs` file.
* It is encouraged to avoids having multiple files named `mod.rs` within a project.
* Every `.rs` file(except for main.rs, lib.rs and other crate root files) should have exactly one mod declaration.
* Use `mod` only once per module, and `use` brings item into scope.

## TODO
- `lib.rs` file usage.
- Create own cargo.toml file with folder(Not sure the actual definition of this)
- Enum file or trait (Where should it be located?)

## Good examples for rust projects
* https://github.com/pemistahl/grex/tree/main
* https://github.com/AbdelilahOu/Rusty-school/tree/main
* https://github.com/superjose/rust-include-files-example/tree/master
