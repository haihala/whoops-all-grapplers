# Development
- For design ideology, see [Philosophy](docs/gameplay_spec/guides/philosophy.md)
- For writing the documentation, see [Writing the spec](docs/gameplay_spec/guides/writing_the_spec.md)

## Setup
Not tested on mac

1. Install rust tools using rustup
2. Install [bevy dependencies](https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md) (linux)
3. Install clang and lld (linux) for faster linking
4. Install visual studio 2019 windows sdk (only for windows)
	1. Newer one may also work
5. Install cargo-make
6. In the `client` directory, start with
	1. Dev mode: `cargo make dev` (see [Dev mode](docs/metagame/modes/dev_mode.md))
	2. Prod mode: `cargo make prod` (see [Versus mode](docs/metagame/modes/versus_mode.md))
7. See `Makefile.toml` for other commands

# Tech used
The project is a git repo on https://github.com/rorawok/whoops-all-grapplers. Github actions is used to run CI (tests and lints).

Non-trivial rust tooling:
- `cargo-udeps` - Notifies of unused dependencies
- `cargo-make` - Runs scripts similar to gnu make
