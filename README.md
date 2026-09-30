# OzdemirGPUIThemePack

A **Fluent UI theme pack for [gpui-kit](https://gpui-kit.com/)** (the component library on top of Zed's [GPUI](https://gpui.rs/)): a Rust-first token library, CSS theme files, and a live-preview gallery app.

> Status: early stage (version stays `0.0.0`). History: this repository began as a cancelled file-manager product (cancelled 2026-09-30) and the Fluent theme infrastructure became the project. Product definition: [PRD.md](PRD.md). The gallery UI is Turkish-first.

## What you get

- **Library crate `ozdemirgpuithemepack`** — Fluent 2 tokens in Rust, a gpui-kit theme bridge, and a CSS theme-file loader:

  ```rust
  use ozdemirgpuithemepack::fluentui::{theme, Accent};

  // Straight from the Rust tokens:
  theme::apply(Accent::Teal, true, Some(window), cx);

  // Or from a generated theme file:
  use ozdemirgpuithemepack::css;
  let config = css::load(std::path::Path::new("themes/FluentDarkTeal.css"))?;
  theme::apply_config(config, Some(window), cx);
  ```

- **16 theme files** (`themes/*.css`, 8 accents × light/dark) as single `:root` custom-property sheets:

  ```css
  /* OzdemirGPUIThemePack — FluentLightBlue */
  :root {
    --name: "FluentLightBlue";
    --mode: light;
    --radius: 4px;
    --radius-lg: 8px;
    --shadow: none;
    --background: #f3f3f3ff;
    --foreground: #000000e4;
    --primary-background: #0066b4ff;
    --primary-hover-background: #0078d4ff;
    /* … */
  }
  ```

  A gpui-kit schema key (`primary.hover.background`) maps to `--primary-hover-background`; colors always normalize to `#RRGGBBAA`. gpui-kit's own `ThemeRegistry` only reads JSON — the CSS path is provided by this crate (`ozdemirgpuithemepack::css`).

- **Gallery app `ThemeGallery`** — every gpui-kit control under the active variant, with live accent/darkness switching (theme preview **is** the product).

## Repository layout

```
src/ozdemirgpuithemepack/   Library crate (tokens, gpui-kit bridge, CSS loader, GenThemes bin)
  src/fluentui/             Fluent 2 design tokens → gpui-kit (Rust-first)
  src/css.rs                lightningcss .css → ThemeConfig loader
src/ThemeGallery/           Gallery app (live theme preview)
themes/                     16 generated CSS theme files (8 accents × light/dark)
.kilo/skills/               Project skills (FluentTheme, GpuiKit)
tr/                         Turkish mirrors of the documentation
```

## Requirements

- Linux with X11 (Wayland: rendering works; see notes below)
- Rust (latest stable)
- System packages (gpui):

  ```sh
  sudo apt install libfontconfig1-dev libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev
  ```

## Build & run

```sh
cargo build --workspace          # build everything
./run.sh                         # build + run the gallery
cargo run -p ThemeGallery        # run the gallery directly
cargo test --workspace           # tests (incl. themes_are_in_sync)
cargo clippy --workspace --all-targets
```

### Fluent theme files

`themes/*.css` are **generated** from the Rust token module — never edit them by hand. After changing `src/ozdemirgpuithemepack/src/fluentui/colors.rs` or `theme.rs`:

```sh
cargo run -p ozdemirgpuithemepack --bin GenThemes
```

The `themes_are_in_sync` test fails if the CSS files drift from the Rust source.

## Window & title bar

The gallery uses gpui-kit's client-side `TitleBar` with `WindowDecorations::Client`, so the system (WM) title bar is replaced by the in-app Fluent title bar. On X11 this requires a running compositor; without one GPUI automatically falls back to server-side decorations (see AGENTS.md → Environment notes).

## Documentation

- [PRD.md](PRD.md) — product requirements (theme pack, open decisions)
- [AGENTS.md](AGENTS.md) — contributor/agent guide (build rules, conventions)
- [tr/](tr/) — Turkish versions of these documents
- Skills: [FluentTheme](.kilo/skills/FluentTheme/SKILL.md), [GpuiKit](.kilo/skills/GpuiKit/SKILL.md)

## Roadmap (open decisions)

1. Theme designer — visual token editor with live preview, built on `ozdemirgpuithemepack::css`
2. crates.io publishing of the library crate
3. Additional design languages (e.g. Material) as development-time variants

## Screenshots

_TBD_

## License

Project licensing TBD. The CSS parser (`lightningcss`) is MPL-2.0; no GPL code enters this repository.
