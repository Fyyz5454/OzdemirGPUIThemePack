//! Embedded theme CSS resources (Java-style `resources/` folder).
//!
//! The generated Fluent theme sheets live in
//! `src/ozdemirgpuithemepack/resources/themes/*.css` and are embedded into
//! the crate at compile time with `include_str!`. Consumers can parse them
//! with [`crate::css::load_str`] — no file paths, no asset management.
//!
//! Regenerate the files with `cargo run -p ozdemirgpuithemepack --bin GenThemes`;
//! the `themes_are_in_sync` test guards against drift between the token
//! module and the embedded sheets.

/// An embedded theme CSS file with its canonical theme name.
pub struct ThemeCssFile {
    /// Theme name, e.g. `"FluentLightBlue"`.
    pub name: &'static str,
    /// The full CSS custom-property sheet.
    pub css: &'static str,
}

/// `FluentDarkBlue.css`
pub const FLUENT_DARK_BLUE: &str = include_str!("../resources/themes/FluentDarkBlue.css");
/// `FluentDarkGreen.css`
pub const FLUENT_DARK_GREEN: &str = include_str!("../resources/themes/FluentDarkGreen.css");
/// `FluentDarkMagenta.css`
pub const FLUENT_DARK_MAGENTA: &str = include_str!("../resources/themes/FluentDarkMagenta.css");
/// `FluentDarkOrange.css`
pub const FLUENT_DARK_ORANGE: &str = include_str!("../resources/themes/FluentDarkOrange.css");
/// `FluentDarkPurple.css`
pub const FLUENT_DARK_PURPLE: &str = include_str!("../resources/themes/FluentDarkPurple.css");
/// `FluentDarkRed.css`
pub const FLUENT_DARK_RED: &str = include_str!("../resources/themes/FluentDarkRed.css");
/// `FluentDarkTeal.css`
pub const FLUENT_DARK_TEAL: &str = include_str!("../resources/themes/FluentDarkTeal.css");
/// `FluentDarkYellow.css`
pub const FLUENT_DARK_YELLOW: &str = include_str!("../resources/themes/FluentDarkYellow.css");
/// `FluentLightBlue.css`
pub const FLUENT_LIGHT_BLUE: &str = include_str!("../resources/themes/FluentLightBlue.css");
/// `FluentLightGreen.css`
pub const FLUENT_LIGHT_GREEN: &str = include_str!("../resources/themes/FluentLightGreen.css");
/// `FluentLightMagenta.css`
pub const FLUENT_LIGHT_MAGENTA: &str = include_str!("../resources/themes/FluentLightMagenta.css");
/// `FluentLightOrange.css`
pub const FLUENT_LIGHT_ORANGE: &str = include_str!("../resources/themes/FluentLightOrange.css");
/// `FluentLightPurple.css`
pub const FLUENT_LIGHT_PURPLE: &str = include_str!("../resources/themes/FluentLightPurple.css");
/// `FluentLightRed.css`
pub const FLUENT_LIGHT_RED: &str = include_str!("../resources/themes/FluentLightRed.css");
/// `FluentLightTeal.css`
pub const FLUENT_LIGHT_TEAL: &str = include_str!("../resources/themes/FluentLightTeal.css");
/// `FluentLightYellow.css`
pub const FLUENT_LIGHT_YELLOW: &str = include_str!("../resources/themes/FluentLightYellow.css");

/// Every embedded theme CSS file, ordered by name.
pub fn themes() -> &'static [ThemeCssFile] {
    &[
        ThemeCssFile {
            name: "FluentDarkBlue",
            css: FLUENT_DARK_BLUE,
        },
        ThemeCssFile {
            name: "FluentDarkGreen",
            css: FLUENT_DARK_GREEN,
        },
        ThemeCssFile {
            name: "FluentDarkMagenta",
            css: FLUENT_DARK_MAGENTA,
        },
        ThemeCssFile {
            name: "FluentDarkOrange",
            css: FLUENT_DARK_ORANGE,
        },
        ThemeCssFile {
            name: "FluentDarkPurple",
            css: FLUENT_DARK_PURPLE,
        },
        ThemeCssFile {
            name: "FluentDarkRed",
            css: FLUENT_DARK_RED,
        },
        ThemeCssFile {
            name: "FluentDarkTeal",
            css: FLUENT_DARK_TEAL,
        },
        ThemeCssFile {
            name: "FluentDarkYellow",
            css: FLUENT_DARK_YELLOW,
        },
        ThemeCssFile {
            name: "FluentLightBlue",
            css: FLUENT_LIGHT_BLUE,
        },
        ThemeCssFile {
            name: "FluentLightGreen",
            css: FLUENT_LIGHT_GREEN,
        },
        ThemeCssFile {
            name: "FluentLightMagenta",
            css: FLUENT_LIGHT_MAGENTA,
        },
        ThemeCssFile {
            name: "FluentLightOrange",
            css: FLUENT_LIGHT_ORANGE,
        },
        ThemeCssFile {
            name: "FluentLightPurple",
            css: FLUENT_LIGHT_PURPLE,
        },
        ThemeCssFile {
            name: "FluentLightRed",
            css: FLUENT_LIGHT_RED,
        },
        ThemeCssFile {
            name: "FluentLightTeal",
            css: FLUENT_LIGHT_TEAL,
        },
        ThemeCssFile {
            name: "FluentLightYellow",
            css: FLUENT_LIGHT_YELLOW,
        },
    ]
}

/// Look up an embedded sheet by theme name (e.g. `"FluentDarkTeal"`).
pub fn get(name: &str) -> Option<&'static str> {
    themes()
        .iter()
        .find(|theme| theme.name == name)
        .map(|theme| theme.css)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_themes_are_complete_and_resolvable() {
        assert_eq!(themes().len(), 16);
        for theme in themes() {
            assert!(theme.css.starts_with("/*"), "{} must embed real CSS", theme.name);
            assert_eq!(get(theme.name), Some(theme.css));
        }
        assert!(get("FluentLightBlue").is_some());
        assert!(get("NoSuchTheme").is_none());
    }
}
