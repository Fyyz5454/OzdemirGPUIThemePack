---
name: FluentTheme
description: Add or modify Fluent theme variants (accent colors, light/dark) in the OzdemirGPUIThemePack gpui-kit codebase. Use when changing Fluent tokens, adding accent colors, editing theme.rs mappings, or regenerating themes/*.css.
---

# FluentTheme — Fluent variant workflow

The Fluent theme system is **Rust-first**: tokens live in `src/ozdemirgpuithemepack/src/fluentui/`, CSS files are derived artifacts. Never edit `themes/*.css` by hand.

## 1. Where things live

| File | Role |
|---|---|
| `src/ozdemirgpuithemepack/src/fluentui/colors.rs` | Fluent 2 token constants (`ResourceDictionary::light()/dark()`), accent palettes (`palette::*`), `Accent` enum (8 accents), helpers `c(argb)`, `hex_string`, `with_alpha` |
| `src/ozdemirgpuithemepack/src/fluentui/theme.rs` | Tokens → gpui-kit `ThemeConfig`: `theme_config(accent, dark)`, `color_entries()` (schema keys — single source for Rust and CSS), `theme_css()` (file bytes), `apply()`/`apply_config()` (runtime, via `Theme::change`), `variants()` (16) |
| `src/ozdemirgpuithemepack/src/css.rs` | lightningcss loader: `parse(&str)`/`load(path)` → `ThemeConfig` (strict `:root` grammar, colors normalize to `#RRGGBBAA`) |
| `src/ozdemirgpuithemepack/src/fluentui/fonts.rs` | Font family priority: "Segoe UI Variable Text" → "Segoe UI Variable" → "Segoe UI" (resolved at runtime, falls back silently) |
| `src/ozdemirgpuithemepack/src/bin/GenThemes.rs` | Writes `themes/Fluent{Light|Dark}{Accent}.css` |
| `themes/*.css` | Generated output — one `:root` custom-property sheet per file, 16 total |

## 2. Color formats — the two traps

- Tokens are authored as **ARGB** u32 (`0xAARRGGBB`). gpui's `rgba()` expects **RGBA byte order** (`0xRRGGBBAA`). Never feed ARGB constants straight into `rgba()` — always go through `c()` in `colors.rs`, which swaps the channels.
- `hex_string(color)` produces `#RRGGBBAA` — the only format gpui-kit's `try_parse_color` accepts for 8-digit hex (it also accepts `#RRGGBB`). The CSS loader always re-emits this canonical form, even for `#fff`/`rgba()` input.

## 3. CSS theme-file grammar

- One `:root` rule per file; multiple `:root` rules are a parse error.
- Meta keys: `--name` (quoted string), `--mode` (`light`\|`dark`), `--radius`/`--radius-lg` (`Npx`), `--shadow` (`none` = flat Fluent controls, `drop` = enabled).
- gpui-kit `ThemeConfigColors` schema key `primary.hover.background` → `--primary-hover-background` (dashes↔dots). `link.hover` → `--link-hover`.
- `font.family` is intentionally absent from files (gpui-kit drops unknown family names; `fonts.rs` resolves at runtime).
- Unknown `--keys` warn on load; missing `--name`/`--mode`/`--background`/`--foreground` are errors.

## 4. Adding a new accent color (end-to-end)

1. `colors.rs`: add `pub fn mycolor() -> AccentColor` with a 7-step swatch (`AccentColor::swatch(darkest, darker, dark, normal, light, lighter, lightest)`).
2. Extend the `Accent` enum: variant + `ALL` array entry + `color()`, `name()` (theme name and file stem), `label()` (Turkish UI label).
3. `ThemeGallery/src/main.rs`: the accent radio group renders from `Accent::ALL`/`label()` automatically; check the variant `Select` still fits 2× variants.
4. `theme.rs`: nothing else — mappings derive from the swatch. Check tests (`variant_count` expects 16 → update to the new count).
5. Run `cargo test -p ozdemirgpuithemepack` (unit tests) — then **last step**: `cargo run -p ozdemirgpuithemepack --bin GenThemes` and commit Rust + CSS together. The `themes_are_in_sync` test fails if you forget.

## 5. Runtime application path

`fluentui::theme::apply(accent, dark, window, cx)` (tokens) or `fluentui::theme::apply_config(config, window, cx)` (e.g. a config from `css::load`):

1. Attaches the runtime-resolved Fluent font family (when installed).
2. Puts the config into `Theme::global_mut(cx).light_theme`/`dark_theme` slot.
3. Calls `Theme::change(mode, window, cx)` — gpui-kit then applies config, syncs base styles, installs text defaults and refreshes the window. Never mutate theme colors outside this path.

The startup default is `Accent::Blue` light (`ThemeGallery/src/main.rs`). The gallery's accent/darkness switcher is the product feature — live switching goes through the same `apply` path.

## 6. Radius, shadow, fonts

- `FLUENT_RADIUS = 4` (controls), `FLUENT_RADIUS_LG = 8` (flyouts/dialogs), `shadow: Some(false)` (Fluent controls are flat) — emitted as `--radius: 4px; --radius-lg: 8px; --shadow: none;`.
- Font families are probed via `cx.text_system().all_font_names()`; CSS theme files intentionally omit `font.family`.

## 7. lightningcss notes

- Pinned `=1.0.0-alpha.72`, MPL-2.0, pure Rust. **All** usage lives in `src/css.rs` — never import lightningcss elsewhere; the parser must stay swappable.
- lightningcss parses `#f3f3f3` into `CssColor::RGBA` and would re-serialize it compacted (`#f3f3f3`, `#fff`, color names). `css.rs` formats colors itself as `#RRGGBBAA`; never rely on lightningcss color serialization.
- `px` lengths arrive as `TokenOrValue::Length(LengthValue::Px(f32))`; idents/strings as lightningcss's own `Token` enum (`Ident`, `String`, `Number`).
