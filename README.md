# war-is-here

Darksiders modding!

## How to use

Still very rudimentary. First, run `dev/inject.sh`. This will spin up a UDP server inside Darksiders that has some basic commands.

All these were tested in Crossroads, so you might want to go there first. Things to try:

```sh
nc -u localhost 12345 -w 1 <<<'spawn_actor vulgrim_chime/vulgrim_chime_medium -4000 -28000 200'
nc -u localhost 12345 -w 1 <<<'spawn_static_object city01_streets ci_shoppingstrip -4000 -24000 50 1'
nc -u localhost 12345 -w 1 <<<'spawn_static_object city01_streets ci_shoppingstrip -4000 -28000 75 0.01'
nc -u localhost 12345 -w 1 <<<'spawn_cube -4000 -28000 150 25'
```

## Development

### Install prerequisites

- [Rust]
- [pre-commit]

[Rust]: https://www.rust-lang.org/
[pre-commit]: https://pre-commit.com/

### Install the pre-commit hook

```sh
pre-commit install
```

This installs a Git hook that runs a quick sanity check before every commit.

### Run the app

```sh
cargo run
```

### Run the tests

```sh
cargo test
```
