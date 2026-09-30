//! Bridge between the Fluent 2 design tokens and the gpui-kit theme system.
//!
//! The token source is the [`super::colors`] module (Rust-first). This module
//! builds gpui-kit [`ThemeConfig`] values from those tokens, applies them
//! via [`Theme::change`], and serializes the same tokens into
//! `themes/*.css` (see [`theme_css`]; the [`super::super::css`] module
//! parses those files back).

use std::rc::Rc;

use gpui_kit::component::theme::{Theme, ThemeConfig, ThemeConfigColors, ThemeMode};
use gpui_kit::{App, Window};

use super::colors::{hex_string, with_alpha, Accent, AccentColor, ResourceDictionary};

/// Fluent control corner radius.
pub const FLUENT_RADIUS: usize = 4;
/// Fluent large-surface (flyout, dialog) corner radius.
pub const FLUENT_RADIUS_LG: usize = 8;

/// An `(accent, dark)` theme variant; 8 accents x 2 brightness levels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FluentVariant {
    pub accent: Accent,
    pub dark: bool,
}

/// All 16 Fluent theme variants.
pub fn variants() -> impl Iterator<Item = FluentVariant> {
    Accent::ALL
        .into_iter()
        .flat_map(|accent| [false, true].map(move |dark| FluentVariant { accent, dark }))
}

/// Theme name, e.g. `FluentDarkTeal`.
pub fn theme_name(accent: Accent, dark: bool) -> String {
    format!(
        "Fluent{}{}",
        if dark { "Dark" } else { "Light" },
        accent.name()
    )
}

/// Theme file stem, e.g. `FluentDarkTeal.css`.
pub fn theme_file_stem(accent: Accent, dark: bool) -> String {
    format!(
        "Fluent{}{}",
        if dark { "Dark" } else { "Light" },
        accent.name()
    )
}

/// Builds a complete gpui-kit theme configuration from Fluent tokens.
///
/// The font family is intentionally left unset: gpui-kit silently drops
/// family names that are not installed, so the family is resolved at
/// runtime inside [`apply_config`].
pub fn theme_config(accent: Accent, dark: bool) -> ThemeConfig {
    let resources = if dark {
        ResourceDictionary::dark()
    } else {
        ResourceDictionary::light()
    };
    let swatch = accent.color();

    ThemeConfig {
        name: theme_name(accent, dark).into(),
        mode: if dark {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        },
        radius: Some(FLUENT_RADIUS),
        radius_lg: Some(FLUENT_RADIUS_LG),
        shadow: Some(false),
        colors: config_colors(&resources, &swatch, dark),
        ..Default::default()
    }
}

/// The color entries of a variant as gpui-kit theme-file schema keys.
///
/// Keys match the serde renames of `ThemeConfigColors` one-to-one; values
/// are `#RRGGBBAA` strings. Both [`config_colors`] (the runtime path) and
/// [`theme_css`] (the file path) derive from this single list, so Rust and
/// CSS can never drift apart.
fn color_entries(
    r: &ResourceDictionary,
    accent: &AccentColor,
    dark: bool,
) -> Vec<(&'static str, String)> {
    // Primary (accent) brush and its hover/active neighbors.
    // Light theme: dark -> normal -> light; dark theme: lighter -> light -> normal.
    let (primary, primary_hover, primary_active) = if dark {
        (accent.lighter, accent.light, accent.normal)
    } else {
        (accent.dark, accent.normal, accent.light)
    };
    let on_accent = r.text_on_accent_fill_color_primary;

    let mut entries: Vec<(&'static str, String)> = Vec::new();
    macro_rules! set {
        ($key:literal, $color:expr) => {
            entries.push(($key, hex_string($color)));
        };
    }

    set!("background", r.solid_background_fill_color_base);
    set!("foreground", r.text_fill_color_primary);
    set!("border", r.card_stroke_color_default);
    set!("input.border", r.control_stroke_color_secondary);

    set!("muted.background", r.control_fill_color_default);
    set!("muted.foreground", r.text_fill_color_secondary);

    set!("accent.background", r.subtle_fill_color_secondary);
    set!("accent.foreground", r.text_fill_color_primary);

    set!("primary.background", primary);
    set!("primary.hover.background", primary_hover);
    set!("primary.active.background", primary_active);
    set!("primary.foreground", on_accent);

    set!("secondary.background", r.control_fill_color_secondary);
    set!("secondary.hover.background", r.control_fill_color_tertiary);
    set!("secondary.active.background", r.control_fill_color_quarternary);
    set!("secondary.foreground", r.text_fill_color_primary);

    set!("button.background", r.control_fill_color_default);
    set!("button.hover.background", r.control_fill_color_secondary);
    set!("button.active.background", r.control_fill_color_tertiary);
    set!("button.foreground", r.text_fill_color_primary);

    set!("button.primary.background", primary);
    set!("button.primary.hover.background", primary_hover);
    set!("button.primary.active.background", primary_active);
    set!("button.primary.foreground", on_accent);

    set!("button.secondary.background", r.control_fill_color_secondary);
    set!("button.secondary.hover.background", r.control_fill_color_tertiary);
    set!(
        "button.secondary.active.background",
        r.control_fill_color_quarternary
    );
    set!("button.secondary.foreground", r.text_fill_color_primary);

    set!("danger.background", r.system_fill_color_critical);
    set!("danger.hover.background", r.system_fill_color_critical);
    set!("danger.active.background", r.system_fill_color_critical);
    set!("danger.foreground", on_accent);
    set!("button.danger.background", r.system_fill_color_critical);
    set!("button.danger.hover.background", r.system_fill_color_critical);
    set!("button.danger.active.background", r.system_fill_color_critical);
    set!("button.danger.foreground", on_accent);

    set!("warning.background", r.system_fill_color_caution);
    set!("warning.hover.background", r.system_fill_color_caution);
    set!("warning.active.background", r.system_fill_color_caution);
    set!("warning.foreground", on_accent);
    set!("button.warning.background", r.system_fill_color_caution);
    set!("button.warning.hover.background", r.system_fill_color_caution);
    set!("button.warning.active.background", r.system_fill_color_caution);
    set!("button.warning.foreground", on_accent);

    set!("success.background", r.system_fill_color_success);
    set!("success.hover.background", r.system_fill_color_success);
    set!("success.active.background", r.system_fill_color_success);
    set!("success.foreground", on_accent);
    set!("button.success.background", r.system_fill_color_success);
    set!("button.success.hover.background", r.system_fill_color_success);
    set!("button.success.active.background", r.system_fill_color_success);
    set!("button.success.foreground", on_accent);

    set!("info.background", accent.normal);
    set!("info.hover.background", primary_hover);
    set!("info.active.background", primary_active);
    set!("info.foreground", on_accent);
    set!("button.info.background", accent.normal);
    set!("button.info.hover.background", primary_hover);
    set!("button.info.active.background", primary_active);
    set!("button.info.foreground", on_accent);

    set!("link", primary);
    set!("link.hover", primary_hover);
    set!("link.active", primary_active);

    set!("selection.background", with_alpha(accent.normal, 0.4));
    set!("caret", r.text_fill_color_primary);
    set!("ring", r.focus_stroke_color_outer);

    set!("title_bar.background", r.solid_background_fill_color_base);
    set!("title_bar.border", r.card_stroke_color_default);
    set!("window.border", r.card_stroke_color_default_solid);

    set!("popover.background", r.solid_background_fill_color_quarternary);
    set!("popover.foreground", r.text_fill_color_primary);

    set!("switch.background", r.control_strong_fill_color_default);
    set!("switch.thumb.background", r.control_solid_fill_color_default);

    set!("slider.background", r.control_fill_color_tertiary);
    set!("slider.thumb.background", r.control_strong_fill_color_default);

    set!("progress.bar.background", primary);

    set!("scrollbar.background", r.control_alt_fill_color_secondary);
    set!("scrollbar.thumb.background", r.control_alt_fill_color_tertiary);
    set!(
        "scrollbar.thumb.hover.background",
        r.control_alt_fill_color_quarternary
    );

    set!("drag.border", accent.normal);
    set!("drop_target.background", r.subtle_fill_color_tertiary);
    set!("overlay", r.smoke_fill_color_default);

    set!("list.background", r.solid_background_fill_color_quarternary);
    set!("list.hover.background", r.subtle_fill_color_secondary);
    set!("list.active.background", r.subtle_fill_color_tertiary);
    set!("list.even.background", r.layer_fill_color_alt);
    set!("list.head.background", r.solid_background_fill_color_quarternary);

    set!("group_box.background", r.layer_fill_color_default);
    set!("group_box.foreground", r.text_fill_color_secondary);

    set!("skeleton.background", r.control_fill_color_default);

    entries
}

/// Produces the colors through gpui-kit's theme-file schema (dotted keys).
///
/// The keys match the serde renames of `ThemeConfigColors` one-to-one, so
/// the Rust side and the `themes/*.css` files flow through the same code
/// path.
fn config_colors(r: &ResourceDictionary, accent: &AccentColor, dark: bool) -> ThemeConfigColors {
    let entries = color_entries(r, accent, dark);
    let map = entries
        .into_iter()
        .map(|(key, value)| (key.to_string(), serde_json::Value::String(value)))
        .collect::<serde_json::Map<_, _>>();

    serde_json::from_value(serde_json::Value::Object(map)).expect("valid Fluent theme colors")
}

/// File content of a variant's `themes/*.css` (a `:root` custom-property
/// sheet; see [`crate::css`] for the grammar).
///
/// Both the generator binary and the sync test use this function, so file
/// contents always derive from the same source.
pub fn theme_css(accent: Accent, dark: bool) -> String {
    let resources = if dark {
        ResourceDictionary::dark()
    } else {
        ResourceDictionary::light()
    };
    let name = theme_name(accent, dark);

    let mut out = String::new();
    out.push_str(&format!("/* OzdemirGPUIThemePack — {name} */\n"));
    out.push_str(":root {\n");
    out.push_str(&format!("  --name: \"{name}\";\n"));
    out.push_str(&format!(
        "  --mode: {};\n",
        if dark { "dark" } else { "light" }
    ));
    out.push_str(&format!("  --radius: {FLUENT_RADIUS}px;\n"));
    out.push_str(&format!("  --radius-lg: {FLUENT_RADIUS_LG}px;\n"));
    out.push_str("  --shadow: none;\n");
    for (key, value) in color_entries(&resources, &accent.color(), dark) {
        out.push_str(&format!("  --{}: {value};\n", key.replace('.', "-")));
    }
    out.push_str("}\n");
    out
}

/// Applies the Fluent theme to the gpui-kit global.
///
/// The configuration is placed into the matching `light_theme`/`dark_theme`
/// slot and activated via [`Theme::change`], so gpui-kit's own theme
/// transition path (base synchronization, text defaults, window refresh)
/// runs unchanged. When a Fluent font family is installed it is attached as
/// the theme family.
pub fn apply(accent: Accent, dark: bool, window: Option<&mut Window>, cx: &mut App) {
    apply_config(theme_config(accent, dark), window, cx);
}

/// Applies an already-built theme configuration (e.g. one loaded from a
/// `themes/*.css` file via [`crate::css`]).
pub fn apply_config(mut config: ThemeConfig, window: Option<&mut Window>, cx: &mut App) {
    config.font_family = super::fonts::resolve_fluent_font(cx);
    let mode = config.mode;
    let config = Rc::new(config);

    {
        let theme = Theme::global_mut(cx);
        if mode.is_dark() {
            theme.dark_theme = config;
        } else {
            theme.light_theme = config;
        }
    }

    Theme::change(mode, window, cx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::Hsla;
    use gpui_kit::component::theme::try_parse_color;

    fn parse(value: &Option<gpui_kit::SharedString>) -> Hsla {
        try_parse_color(value.as_deref().unwrap()).unwrap()
    }

    fn rgba(hex: u32) -> Hsla {
        gpui_kit::rgba(hex).into()
    }

    fn rgba_from(color: gpui_kit::Rgba) -> Hsla {
        color.into()
    }

    #[test]
    fn variant_count() {
        assert_eq!(variants().count(), 16);
    }

    #[test]
    fn light_blue_core_tokens() {
        let config = theme_config(Accent::Blue, false);
        assert_eq!(config.name, "FluentLightBlue");
        assert_eq!(config.mode, ThemeMode::Light);
        assert_eq!(config.radius, Some(4));
        assert_eq!(config.radius_lg, Some(8));
        assert_eq!(config.shadow, Some(false));

        let blue = Accent::Blue.color();
        assert_eq!(
            parse(&config.colors.background),
            rgba(0xf3f3f3ff)
        );
        assert_eq!(parse(&config.colors.foreground), rgba(0x000000e4));
        assert_eq!(parse(&config.colors.primary), rgba_from(blue.dark));
        assert_eq!(parse(&config.colors.primary_hover), rgba_from(blue.normal));
        assert_eq!(parse(&config.colors.link), rgba_from(blue.dark));
        assert_eq!(
            parse(&config.colors.title_bar),
            rgba(0xf3f3f3ff)
        );
        assert_eq!(parse(&config.colors.popover), rgba(0xffffffff));
        assert_eq!(parse(&config.colors.danger), rgba(0xc42b1cff));
        assert_eq!(
            parse(&config.colors.muted_foreground),
            rgba(0x0000009e)
        );
        // selection fill = accent normal + alpha 0.4
        let selection = parse(&config.colors.selection);
        assert!((selection.a - 0.4).abs() < 0.01);
    }

    #[test]
    fn dark_blue_core_tokens() {
        let config = theme_config(Accent::Blue, true);
        assert_eq!(config.name, "FluentDarkBlue");
        assert_eq!(config.mode, ThemeMode::Dark);

        let blue = Accent::Blue.color();
        assert_eq!(
            parse(&config.colors.background),
            rgba(0x202020ff)
        );
        assert_eq!(parse(&config.colors.foreground), rgba(0xffffffff));
        assert_eq!(parse(&config.colors.primary), rgba_from(blue.lighter));
        assert_eq!(parse(&config.colors.link), rgba_from(blue.lighter));
        assert_eq!(parse(&config.colors.title_bar), rgba(0x202020ff));
    }

    #[test]
    fn accents_follow_swatches() {
        for accent in Accent::ALL {
            let swatch = accent.color();
            let light = theme_config(accent, false);
            let dark = theme_config(accent, true);
            assert_eq!(parse(&light.colors.primary), rgba_from(swatch.dark));
            assert_eq!(parse(&dark.colors.primary), rgba_from(swatch.lighter));
            assert_eq!(parse(&light.colors.progress_bar), rgba_from(swatch.dark));
        }
    }

    #[test]
    fn names_and_files() {
        assert_eq!(theme_name(Accent::Blue, false), "FluentLightBlue");
        assert_eq!(theme_name(Accent::Yellow, true), "FluentDarkYellow");
        assert_eq!(theme_file_stem(Accent::Teal, true), "FluentDarkTeal");
        assert_eq!(theme_file_stem(Accent::Red, false), "FluentLightRed");
    }

    #[test]
    fn theme_css_roundtrip() {
        let css = theme_css(Accent::Teal, true);
        let loaded = crate::css::parse(&css).unwrap();

        let expected = theme_config(Accent::Teal, true);
        assert_eq!(loaded.name, expected.name);
        assert_eq!(loaded.mode, expected.mode);
        assert_eq!(loaded.radius, expected.radius);
        assert_eq!(loaded.radius_lg, expected.radius_lg);
        assert_eq!(loaded.shadow, expected.shadow);
        assert_eq!(
            serde_json::to_value(&loaded.colors).unwrap(),
            serde_json::to_value(&expected.colors).unwrap(),
            "every color entry survives the CSS round trip"
        );
    }

    #[test]
    fn theme_css_has_no_rgb_fallbacks() {
        // Fully opaque and semi-transparent colors must stay 8-digit hex.
        let css = theme_css(Accent::Blue, false);
        assert!(css.contains("--background: #f3f3f3ff;"));
        assert!(css.contains(&format!(
            "--selection-background: {};",
            hex_string(with_alpha(Accent::Blue.color().normal, 0.4))
        )));
        assert!(!css.contains("rgb("));
    }

    /// Are the `themes/*.css` files byte-identical to the Rust token module?
    /// Drift olursa: `cargo run -p ozdemirgpuithemepack --bin GenThemes`
    #[test]
    fn themes_are_in_sync() {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../themes");
        let mut files = 0;
        for variant in variants() {
            let path = dir.join(format!(
                "{}.css",
                theme_file_stem(variant.accent, variant.dark)
            ));
            let content = std::fs::read_to_string(&path).unwrap_or_else(|_| {
                panic!(
                    "missing theme file: {} — run `cargo run -p ozdemirgpuithemepack --bin GenThemes`",
                    path.display()
                )
            });
            assert_eq!(
                content,
                theme_css(variant.accent, variant.dark),
                "file does not match the Rust module: {}",
                path.display()
            );
            files += 1;
        }
        assert_eq!(files, 16, "unexpected file count in themes/");
    }
}
