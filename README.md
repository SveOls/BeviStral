# BeviStral

A Bevy 0.19 UI menu application driven by states and nested substates:
`AppState` → `MainMenu` (`Root` / `Settings`) → `SettingsTab`
(`Audio` / `Video` / `Controls`). Each state spawns its screen on enter
and despawns it on exit.

## System dependencies (Debian/Ubuntu)

Bevy's default `wayland` feature needs `pkg-config` and the Wayland/X11
development libraries at build time. Install everything with:

```sh
sh scripts/setup.sh
```

or manually:

```sh
sudo apt-get install -y --no-install-recommends \
    pkg-config libwayland-dev libxkbcommon-dev libx11-dev \
    libxcursor-dev libxrandr-dev libxi-dev libxinerama-dev \
    libudev-dev libasound2-dev
```

## Build

```sh
cargo check -j 2   # low parallelism to limit memory usage
cargo run           # full build takes 10+ minutes on first run
```
