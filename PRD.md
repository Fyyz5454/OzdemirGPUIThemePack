# PRD — OzdemirGPUIThemePack Product Requirements

Status: living document · Version: pre-release, repo version stays **0.0.0** · Last updated: 2026-09-30

## 1. Product identity

**OzdemirGPUIThemePack** is a **Fluent UI theme pack for [gpui-kit](https://gpui-kit.com/)**: a Rust-first token library, CSS theme files, and a live-preview gallery application. GPU-rendered apps built on gpui-kit get an authentic Fluent 2 look by depending on one crate.

Project history: the repository began as a Linux file-manager product that was cancelled on 2026-09-30; the Fluent theme infrastructure survived and became this project. The former file-manager backend (pure-Rust engine stack) was deleted outright by decision.

## 2. Goals & non-goals

### Goals

- **Library crate `ozdemirgpuithemepack`**: Fluent 2 design tokens in Rust (`fluentui::colors`), the gpui-kit bridge (`fluentui::theme::apply`), and a CSS theme-file loader (`css::parse`/`css::load`).
- **CSS theme files**: `themes/*.css` — one `:root { --key: value; }` sheet per variant, 8 accents × light/dark. **No JSON theme files** (decision 1).
- **Gallery application `ThemeGallery`**: live switching across all 16 variants over a gpui-kit widget showcase; theme preview is the product, not a debug tool.
- Deterministic generation: files are produced by `GenThemes` from the Rust tokens and byte-checked by the `themes_are_in_sync` test.

### Non-goals (explicitly excluded)

- A file manager, file operations, or any engine work (cancelled product).
- JSON theme output or gpui-kit `ThemeRegistry` directory wiring (see decision 1).
- A plugin/extension system.

## 3. Feature set

Confirmed:

- 16 Fluent variants (Blue, Red, Green, Teal, Purple, Magenta, Orange, Yellow × light/dark) with correct Fluent 2 neutral + accent ramps.
- Typography bridge: Segoe UI Variable family chain resolved at runtime with silent fallback.
- Live theme application through gpui-kit's `Theme::change` (base sync, text defaults, window refresh).
- CSS loader with strict grammar: colors normalize to `#RRGGBBAA`, unknown properties warn, missing required properties error.
- Gallery UI (Turkish strings): accent radio group, dark-mode switch, variant select, default-reset, plus the gpui-kit widget showcase.

Roadmap:

1. **Theme designer** — visual token editor (color pickers, radius, font), live preview, open/save `.css` theme files on top of the `css` module. Deferred by decision (2026-09-30); architecture must build on `ozdemirgpuithemepack::css`, never on a parallel format.
2. crates.io publishing of the library crate.
3. Possible additional design languages (e.g. Material) as separate token modules — development-time variants, decided per release.

## 4. Architecture

- `src/ozdemirgpuithemepack` — the library. `src/fluentui/` (token source of truth: `colors.rs`, `typography.rs`, `fonts.rs`, `theme.rs`), `src/css.rs` (lightningcss-based `.css` → `ThemeConfig` loader), `src/bin/GenThemes.rs` (writes `themes/*.css`).
- `src/ThemeGallery` — the gallery binary; applies themes via the library only.
- `themes/` — 16 generated CSS files. **Generated, never hand-edited.**
- Theme file grammar: gpui-kit `ThemeConfigColors` schema key `primary.hover.background` → `--primary-hover-background`; meta keys `--name`, `--mode` (light|dark), `--radius`/`--radius-lg` (px), `--shadow` (none|drop); `font.family` intentionally absent (runtime-resolved).

## 5. Decisions (resolved)

1. **CSS over JSON — RESOLVED (2026-09-30)**: gpui-kit's `ThemeRegistry` only reads `*.json` (`registry.rs` scans JSON `ThemeSet`s); there is no CSS support upstream. Instead of patching the dependency, the pack ships its own CSS entry path (`ozdemirgpuithemepack::css`) and drops JSON theme files entirely. `register_theme_dir` was removed. Reintroducing JSON output requires a new decision.
2. **CSS parser: `lightningcss` — RESOLVED (2026-09-30)**: user pick of a ready-made full CSS parser over a hand-rolled tokenizer or a thin `cssparser` layer. Pinned `=1.0.0-alpha.72` (alpha accepted); all usage isolated behind `src/css.rs` so the parser can be swapped without touching callers. Note: lightningcss normalizes colors (`#fff`, `rgba()`, hex compaction), so the loader re-emits every color as canonical `#RRGGBBAA`.
3. **Product shape: library + gallery (+ designer later) — RESOLVED (2026-09-30)**.
4. **FM cancellation — RESOLVED (2026-09-30)**: uncommitted engine work (fs/archive/thumbnail engine code, workspace engine dependencies) deleted without an archive branch, per user decision.

## 6. Constraints & environment notes

- `lightningcss` is 1.0.0-alpha and moves: keep the exact pin, keep every use behind `ozdemirgpuithemepack::css` (MPL-2.0, pure Rust, built on Firefox/Servo's `cssparser`).
- Version stays 0.0.0 until the designer round stabilizes the file format.
- X11: client-side decorations need a running compositor (`xprop -root _NET_WM_CM_S0`); without one gpui falls back to server-side decorations. On Wayland, client-side window positioning is impossible by protocol.
- gpui embeds its own wgpu (blade) renderer; no extra GPU setup.

## 7. Open decisions (each to be resolved in a dedicated Q&A round, one question at a time)

1. **Theme designer MVP scope** — which token groups are editable, and does it write back into `themes/` or a user directory?
2. **crates.io publishing** — timing, name availability, semver start.
3. **Additional design languages** — whether Material (or others) becomes a second token module.
