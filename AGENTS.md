# AGENTS.md

## Before building

Run the dependency setup script before any `cargo` command in a fresh
sandbox or session:

```sh
sh scripts/setup.sh
```

Bevy 0.19 (default features include `wayland`) runs `pkg-config` for
`wayland-client` during its build script, so compilation fails with
`Could not run pkg-config ... wayland-client` until the system dev
libraries are installed. The script installs exactly what is needed and
is idempotent.

## Build conventions

- Compile with low parallelism to limit memory usage:
  `cargo check -j 2`
- `cargo check -j 2` is the verification gate; a full `cargo build` is
  only needed for a runnable binary and takes 10+ minutes.
- Do not change the Bevy feature set in `Cargo.toml`; the default
  features are intentional.

## Project structure

- `src/main.rs` — the entire application: a UI menu driven by states and
  nested substates (`AppState` → `MainMenu` → `SettingsTab`). Each state
  spawns its screen on enter and despawns it on exit via tagged marker
  components.
- `src/io/` — PLC/IOServer ingestion layer: `source.rs` defines the
  `IoSource` trait and tag types; `simulated.rs` fabricates tag values for
  testing; `mod.rs` hosts `IoPlugin`, which owns the I/O thread, a
  channel-drained `IoTagStore`, and the throttled UI refresh. Real backends
  implement `IoSource` and replace the simulated one in `main.rs`.
- UI only; no game logic.
