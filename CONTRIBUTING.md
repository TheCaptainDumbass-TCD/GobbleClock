# Contributing to Gobble Clock

Guidance for anyone working on Gobble Clock. Read this before changing code.

## Project

Gobble Clock is a standalone desktop app for Debian 13 and Ubuntu that plans a slow cooked Thanksgiving turkey from its weight and the carving time. It replaces a hand made spreadsheet. It runs entirely on the local machine and opens no network connection.

- Author and maintainer: Dennis Ayotte <dennis@thecaptaindumbass.com>
- License: GPL-3.0-or-later
- Package name: `gobbleclock`, app ID `dev.quietwire.gobbleclock`
- Stack: Tauri 2 desktop shell, Leptos (Rust/WebAssembly) UI, JSON files for storage
- Public repository: https://github.com/TheCaptainDumbass-TCD/GobbleClock

## Method rules

The defaults are the method from howtocookathanksgivingturkey.com and the original spreadsheet. Do not change them without Dennis.

- 60 minutes out of the refrigerator, 20 minutes prep while preheating to 475°F
- 20 minutes at 475°F, then 250°F at 20 minutes per pound, timed from the moment the oven is turned down
- The oven door stays closed during the slow cook so the turkey is never shocked
- Lid on 15 minutes after turning down, off for the last 20 minutes, rest 20 minutes
- Weights round to the nearest pound: under half rounds down, half or more rounds up
- Every schedule, printout and export carries the thermometer reminder: 170°F breast, 180°F thigh
- Warn above 20 pounds
- Advanced settings may change times. The whole plan is rebuilt from them

## UI rules

- A button with nothing to do is disabled, grayed out and has a tooltip that says why.
- Tones: pumpkin for oven steps, harvest gold for hands on steps, cranberry for checks, sage for the finish.
- Stopping a cook or deleting a year needs a second confirmation click.
- Every change must look right in the light and dark themes and on the printed checklist.

## Writing rules

These apply to docs, UI text, commit messages, changelog entries and package metadata.

- No em dashes or en dashes. Use commas, colons or separate sentences.
- No Oxford commas: write "A, B and C".
- American spelling: color, labeled, gray.
- Plain words and short sentences.

## Layout

| Crate | Purpose |
|---|---|
| `gc-core` | Schedule engine, rounding, thaw planner, Thanksgiving dates, history seed, text and calendar export |
| `gobbleclock` | Tauri 2 desktop shell, commands, JSON storage and step reminders |
| `gc-ui` | Leptos user interface and print sheet |

## Build and test

```sh
cargo test --workspace --exclude gc-ui
cargo clippy --workspace --exclude gc-ui
cargo clippy -p gc-ui --target wasm32-unknown-unknown
cd crates/gobbleclock && cargo tauri build
```

The package is written to `target/release/bundle/deb/`.

Things that have caused trouble before:

- In Leptos `view!` attributes, wrap an `if` expression that contains `>` in braces.
- `class:name=` needs a closure or signal, not a plain bool. Use `class:on=move || on`.
- Workspace path dependencies carry no version number, or a version bump breaks `cargo metadata`.
- Tauri command arguments arrive in camelCase, so frontend argument structs use `#[serde(rename_all = "camelCase")]` when a name has two words.
- On a headless test machine the window may never appear because GTK waits on the D-Bus session bus. Start the app inside `dbus-run-session`.

## Storage

`settings.json`, `history.json` and `active.json` in the app data directory. Settings and history use `#[serde(default)]` so older files keep loading. Writes go through a temporary file and a rename.

## Releases

Semantic versioning, 0.x until the first public release, which will be 1.0.0.

1. Bump `version` in the root `Cargo.toml`.
2. Add a section to `CHANGELOG.md`.
3. Add a `<release>` entry to `crates/gobbleclock/dev.quietwire.gobbleclock.metainfo.xml`.
4. Run the tests, clippy and the WebAssembly check.
5. Commit, create an annotated tag `vX.Y.Z` and build the `.deb`.
6. Push `main` and the tag, then attach the `.deb` and `SHA256SUMS` to a GitHub release.
