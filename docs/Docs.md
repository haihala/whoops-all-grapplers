# Development
- For design ideology, see [[philosophy]]
## Setup
Not tested on mac

1. Install rust tools using rustup
2. Install [bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) (linux)
3. Install clang and lld (linux) for faster linking
4. Install visual studio 2019 windows sdk (only for windows)
	1. Newer one may also work
5. Install cargo-make
6. In the `client` directory, start with
	1. Dev mode: `cargo make dev` (see [[Dev mode]])
	2. Prod mode: `cargo make prod` (see [[Versus mode]])
7. See `Makefile.toml` for other commands

# Tech used
The project is a git repo on https://github.com/rorawok/whoops-all-grapplers. Github actions is used to run CI (tests and lints).

Non-trivial rust tooling:
- `cargo-udeps` - Notifies of unused dependencies
- `cargo-make` - Runs scripts similar to gnu make
