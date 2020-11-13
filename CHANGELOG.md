# Changelog

## v0.3.0 – 2020-11-12

### Changed

- Triggers overhaul!
  - Smoother lines
  - Fake 3D effect to help with depth perception
  - Each trigger/label pair is a different color so you can tell them apart
  - Labels use a better font
  - Labels no longer overlap each other
  - Cylinders and spheres can be toggled on/off
  - Split the overlay and the world objects into two different commands (/draw_triggers and /mark_triggers)
  - Improved FPS

## v0.2.0 – 2020-11-06

### Added

- In /show_triggers, try to choose a better set of objects to display
- FPS counter (/fps)
- /move_player can now take absolute or relative coordinates (depending on whether the keyword "by" is present)
- Display a scolling log of various engine events (/log_events)

### Fixed

- Fix fps drop when the console is visible
- Fix occasional fps drop when show_triggers is enabled

## v0.1.2 – 2020-04-19

- Restore access to The Amazing Secret.

## v0.1.1 – 2020-04-18

- `/show_triggers` now also draws load regions

## v0.1.0 – 2020-03-14

- Fixed a crash in `/show_triggers`
- Tweaked how `/show_triggers` decides which triggers to draw

## v0.0.2 – 2020-03-12

- Renamed `/pretend_editor` to `/editor_mode`

## v0.0.1-alpha – 2020-03-02

- Initial release
