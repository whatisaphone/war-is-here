# war-is-here

Darksiders modding!

## How to use

Still very rudimentary. First, run `dev/inject.sh`. This will spin up a UDP server inside Darksiders that has some basic commands.

All these were tested in Crossroads, so you might want to go there first. Things to try:

### Commands

Commands are sent over UDP. For an easy way to send them, install `netcat` (`nc`) and run:

```sh
nc -u localhost 12345 -w 1 <<<'your_command_here'
```

#### `load_map_menu`

Show a window that lets you teleport to any area in the game.

```
load_map_menu
```

#### `move_player`

#### `pickup_item`

Gives the player any `Item`.

```
pickup_item bfg_sentinel/weapon_bfg_sentinel
pickup_item weapon_enhancements/common_bane
```

#### `pretend_editor`

#### `show_collision`

#### `show_triggers`

Show triggers in the world.

```
show_triggers
```

#### `spawn_humans`

#### `spawn_object`

Spawn an `Actor` class into the world.

```
spawn_actor vulgrim_chime/vulgrim_chime_medium -4000 -28000 200
spawn_actor ci_streetpropsset01/ci_streetlight_r -4000 -28000 0
```

#### `spawn_static_object`

Spawn an `o3d` resource into the world.

```
spawn_static_object city01_streets ci_shoppingstrip -4000 -24000 50 1
spawn_static_object city01_streets city01_glass2_04 -4000 -28000 180 1
```

You can also spawn objects that are part of this mod (not included with the original game):

```
spawn_static_object city01_streets gritty_cube -4000 -28000 180 1
```

#### `teleport`

Send the player to any area in the game.

```
teleport overworld CI_01
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
