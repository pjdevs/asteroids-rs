# Asteroid-rs

This project aims to be a basic game to get famialiar with the possibilities of Bevy (and Rust at the same time).

## Features

- [x] Basic main menu state
- [x] Basic game state
- [x] One or two local players
- [x] Keyboard and controller inputs for players
- [x] Dynamic asset loading with `bevy_asset_loader`
- [x] Basic custom physcis and collisions
- [x] Basic spritesheet animations
- [x] Basic gameplay mechanics (endless mode shooter, score, spawners, ...)
- [ ] More gameplay mechanics (events, ...)
- [ ] Collectible bonuses 
- [ ] Pause menu
- [x] Cheat features and debug UI with `bevy_editor_egui` and `egui`

## Structure

The game is organized as modular plugins for each feature/mechanic/behavior. Every plugin should be independant when possible and only rely on core `States` of the game to be able to describe systems, assets, and schedule inside the plugin (without exposing `SystemSets` and order everything in another plugin).

In the end the game is just a `PluginGroup` that groups all game plugins.

## Build and Run

To run the project in dev mode (debug cheats, asset hot reloading, dynamic linking, ...) :
```
cargo run --features dev
```

To run the project in release or debug mode without dev features :
```
cargo run --profile debug/release
```

To distribute the game with maximum optimization enabled :
```
cargo build --profile distribution -F tracing/release_max_level_off -F log/release_max_level_off
```

## License

Licensed under MIT license ([LICENSE](LICENSE.md) or [https://opensource.org/license/MIT](https://opensource.org/license/MIT))

Assets in the `asteroid-rs/assets` are all public domain.

## Contribution

Anyone is free to contribute to add "intersting" feature to show off more Bevy/Rust features.

Constributors :

- pjdevs