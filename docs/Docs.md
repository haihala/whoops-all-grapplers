# Tech used
## Code
Code is written in [[Rust]] using the [[Bevy]] game engine.

Code is using nightly rust to gain access to [drain filter](https://doc.rust-lang.org/std/vec/struct.DrainFilter.html). Has caused issues in the past, but fine for the most part. Version is bumped occasionally.

Other crates used:
- `bevy-inspector-egui` - unity like inspector for components
- `bitflags` - Easily manageable flag container. Could maybe be replaced with a simple struct and bool fields, but at the moment this is easier.
- `strum` and `strum_macros` for iterating over enums

## Environment
The project is a git repo on https://github.com/rorawok/whoops-all-grapplers. [[Make]] is used to manage build scripts. [[Jenkins]] is configured to run CI (tests and lints). It's however currently not operational as the key has expired and needs to be reset.

Rust specific tooling:
- [[Cargo]] manages the whole project. [[Make]] is just shorthands for [[Cargo]] commands.
- `rust-clippy` - Godly linter. Honestly a great tool for learning about methods.
- `cargo-nextest` - Tool that runs tests in parallel
- `rustfmt` - Code formatter
- `cargo-udeps` - Notifies of unused dependencies

## Docs
Written with [[Obsidian]], mounting the docs folder, format is close to [[PARA]], but not an exact match.

# Development
## Setup
Not tested on mac

1. Install rust tools using rustup
2. Install additional graphics dev libraries (only on linux)
3. Install clang and lld (only for linux) for faster linking
4. `make install` - Installs dependencies like udeps and nextest
5. `make run`

Having a controller is highly recommended. Currently game starts in pre-round, press enter on the keyboard to start the round. The controller should be picked up and start moving one of the characters on the screen.

## Best practices
- Read through the makefile, it's not long and contains a lot of useful tools
- Add a git hook that runs `make check` before pushing
- For design ideology, see [[philosophy]]

## Test
Tests are divided into unit and integration tests. Unit tests are scattered around and ran with `make test` using nextest (a tool to run tests for separate crates in parallel). Integration tests live in their own crate and are ignored by default. The system for integration tests could use some more work and they are quite barebones at the moment. Ran integration tests with  `make integration`.
