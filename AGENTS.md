# AGENTS.md — Contributor & Agent Guide

Ground rules for humans and coding agents working in this repository.

## What this project is

**OzdemirGPUIThemePack** — a **Fluent UI theme pack for gpui-kit 0.6**: a Rust-first token library (`ozdemirgpuithemepack`), CSS theme sheets embedded in the crate (`resources/themes/*.css`), and a live-preview gallery app (`ThemeGallery`). See [PRD.md](PRD.md) for the product definition, resolved decisions, and open questions.

History: the repo started as a cancelled file-manager product; its engine code was deleted. Do not reintroduce file-manager/engine work.

## Commands

| Task | Command |
|---|---|
| Build everything | `cargo build --workspace` |
| Run the gallery | `./run.sh` or `cargo run -p ThemeGallery` |
| Tests | `cargo test --workspace` |
| Lint | `cargo clippy --workspace --all-targets` (must be warning-free) |
| Regenerate theme CSS | `cargo run -p ozdemirgpuithemepack --bin GenThemes` |

System dependencies (one-time): `sudo apt install libfontconfig1-dev libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev`.

## Architecture map

- `src/ozdemirgpuithemepack` — the **library crate**.
  - `src/fluentui/` — **the token source of truth**: `colors.rs` (Fluent 2 tokens, ARGB u32 → gpui `Rgba`), `typography.rs`, `fonts.rs` (Segoe UI family chain), `theme.rs` (tokens → gpui-kit `ThemeConfig`; `apply`/`apply_config` via `Theme::change`; `theme_css` deterministic CSS generator).
  - `src/css.rs` — CSS theme-file loader (lightningcss): `:root` custom properties → `ThemeConfig`; colors normalize to `#RRGGBBAA`.
  - `src/bin/GenThemes.rs` — writes `resources/themes/Fluent{Light|Dark}{Accent}.css`.
- `src/ThemeGallery` — the gallery app (AtlantaFX-sampler layout): `layout/` (title bar with the live theme switcher + search palette, sidebar navigation) and `pages/` (registry + `general/` + `components/`, one page per gpui-kit control family); English UI strings.
- `src/ozdemirgpuithemepack/resources/themes/` — 16 generated files (`Fluent{Light|Dark}{Accent}.css`), embedded via `include_str!` and exposed by `src/resources.rs`. **Generated, never hand-edited.**

## Language & tooling rules

1. **Reply in the language of the incoming prompt.** A Turkish prompt gets a Turkish answer, an English prompt an English answer, and so on.
2. **LSP, the debugger MCP, and the headless-browser MCP are mandatory tools.** Use LSP for code navigation and refactors, the debugger MCP when diagnosing failures, and the headless browser (Playwright MCP) for anything web-related. If a required tool/MCP is unavailable in the session, say so explicitly and proceed best-effort.
3. **This is a global project: code and naming are English.** Identifiers, comments, file names, commit messages, and user-facing UI strings are all in English.
4. **Web research goes through the headless browser**, issuing queries in **both English and Chinese**.

## Hard rules

1. **Rust-first, CSS last.** All Fluent token changes start in `src/ozdemirgpuithemepack/src/fluentui/`. CSS theme files are produced **at the very end** by running `GenThemes`. The `themes_are_in_sync` test enforces byte-equality — never bypass it.
2. **Dry-run before renames/moves.** List the affected references first (e.g. `grep -rn "old-name" --include="*.toml" --include="*.rs"`), show them, then act, then rebuild + test.
3. **One question at a time** when interviewing the user about design decisions. Never batch questions.
4. **No JSON theme files.** gpui-kit's `ThemeRegistry` only reads JSON; this pack deliberately replaced JSON with CSS loaded through `ozdemirgpuithemepack::css` (PRD decision 1). Never reintroduce JSON theme output, `register_theme_dir`, or direct `ThemeSet` serialization without a new user decision.
5. **UI strings are English** in the gallery (global project; keep user-visible strings in the UI layer English).
6. **Window/title bar**: gpui-kit `TitleBar` + `WindowOptions { window_decorations: Some(WindowDecorations::Client), ..TitleBar::window_options() }`. Do not reintroduce the system title bar.
7. **Commits**: messages in **English**; repo-local git identity is `omer <omer@localhost>` (never change global git config). Only commit when the user asks.
8. **No GPL dependencies.** Reference projects are for inspiration only; verify per-crate licenses when adding dependencies (`lightningcss` is MPL-2.0, pure Rust, pinned `=1.0.0-alpha.72`; the gpui-kit stack is Apache-2.0). The project's own license is AGPL-3.0-or-later (see `LICENSE`).

## Environment notes (Linux/X11)

- gpui links X11/wayland directly (no winit). Client-side decorations (no WM title bar) require a **running compositor** + WM support for `_GTK_FRAME_EXTENTS`; without a compositor, gpui silently falls back to server-side decorations. Check with `xprop -root _NET_WM_CM_S0`.
- On Wayland, client-side window positioning is impossible by protocol; do not attempt "always centered" logic there.
- gpui embeds its own wgpu renderer (blade/wgpu); no additional GPU setup is needed.

## Theme workflow (short version)

Details in [.kilo/skills/FluentTheme/SKILL.md](.kilo/skills/FluentTheme/SKILL.md). Summary: edit `colors.rs`/`theme.rs` → `cargo test -p ozdemirgpuithemepack` → regenerate CSS with `GenThemes` → commit Rust + CSS together.

## When something is blocked

If a requirement conflicts with gpui/gpui-kit capabilities, verify against the vendored sources (read `~/.cargo/registry/src/*/gpui-component-0.6.6/…` or `gpui-kit-0.6.6/…`) or https://gpui-kit.com docs, and report trade-offs to the user instead of inventing workarounds that patch dependencies.
