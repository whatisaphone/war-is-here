# <img src="docs/war.png" width="40%" align="right"> war-is-here

Darksiders modding!

## How to use

Make sure you have the Steam version of Darksiders Warmastered edition. Other versions probably won't work.

- Download the latest version from [here][download].
- Extract `horseman.exe` and `aether.dll` to the same directory.
- Start Darksiders Warmastered Edition.
- Run `horseman.exe`.
- Now inside the game, you should be able to hit the tilde key (\`) and see a console appear inside the game. Type `/help` for a list of commands.

[download]: https://github.com/whatisaphone/war-is-here/releases

### Commands

- **/clear**

- **/console**

- **/draw_triggers**

  Draw an overlay revealing the closest few trigger volumes.

  See also:

  - /mark_triggers

- **/draw_triggers_round**

  Turn cylinders and spheres on/off. This only has an effect when `draw_triggers` is enabled.

- **/dump_hstrings**

  Dump the string table (hashes and their associated strings) to a file.

- **/editor_mode** – Run this before entering a game, and the world will load in "editor mode". Most enemies/items are deactivated, and the minimap spawns in the sky!

- **/fps** – Display an FPS counter.

- **/help**

- **/infinite_jump** – Gives the player infinite jump height, as long as the button is held down.

- **/load_map_menu** – Shows the secret window the game developers put in to let them teleport around the game world.

- **/load_package**

- **/log_events**

- **/mark_triggers**

  Add objects to the game world which reveal the location of all currently spawned trigger volumes.

  See also:

  - /draw_triggers

- **/move_player**

  Move to a position on the map:

  ```
  /move_player -4000 -28000 100
  ```

  Move relative to your current position:

  ```
  /move_player by 500 0 0
  ```

- **/pickup_item** – Gives the player any `Item`. Examples:

  ```
  /pickup_item bfg_sentinel/weapon_bfg_sentinel
  /pickup_item weapon_enhancements/common_bane
  ```

- **/show_bird_kills**

  Draw a kill counter during the autoscroller. This is the same counter used for the Aerial Predator achievement (which you earn for 160 kills).

- **/show_collision**

  This is currently broken and will crash the game.

- **/show_player_pos**

- **/shutdown**

- **/spawn_humans** – Spawns a group of mouth breathers into the world.

- **/spawn_object** – Spawns any `WorldObject` into the world. Examples:

  ```
  /spawn_object vulgrim_chime/vulgrim_chime_medium -4000 -28000 200
  /spawn_object ci_streetpropsset01/ci_streetlight_r -4000 -28000 0
  ```

- **/spawn_static_object** – Spawns any `o3d` resource into the world. Examples:

  ```
  /spawn_static_object city01_streets ci_shoppingstrip -4000 -24000 50 1
  /spawn_static_object city01_streets city01_glass2_04 -4000 -28000 180 1
  ```

  You can also spawn objects that are part of this mod (not included with the original game):

  ```
  /spawn_static_object city01_streets gritty_cube -4000 -28000 180 1
  ```

- **/teleport** – Sends the player to any area in the game. Example:

  ```
  /teleport overworld CI_01
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

### Set the toolchain

```sh
rustup override set nightly-2020-10-07-i686-pc-windows-msvc
```

### Run the app

```sh
dev/inject.sh
```

### Run the tests

```sh
cargo test
```
