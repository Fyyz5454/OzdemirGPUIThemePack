//! Loads OzdemirGPUIThemePack CSS theme files into gpui-kit theme configs.
//!
//! A theme file is a single `:root` rule of CSS custom properties:
//!
//! ```css
//! /* OzdemirGPUIThemePack — FluentLightBlue */
//! :root {
//!   --name: "FluentLightBlue";
//!   --mode: light;
//!   --radius: 4px;
//!   --radius-lg: 8px;
//!   --shadow: none;
//!   --background: #f3f3f3ff;
//!   --foreground: #000000e4;
//!   /* … */
//! }
//! ```
//!
//! Mapping rules:
//!
//! - A gpui-kit `ThemeConfigColors` schema key (`primary.hover.background`)
//!   becomes the custom property `--primary-hover-background`.
//! - Colors are always re-emitted as `#RRGGBBAA` regardless of the input
//!   spelling (`#fff`, `#ffffff`, `rgba(…)` all normalize to 8-digit hex,
//!   the format gpui-kit's `try_parse_color` accepts).
//! - Meta keys: `--name` (theme name), `--mode` (`light` | `dark`),
//!   `--radius` / `--radius-lg` (`px`), `--shadow` (`none` = flat,
//!   `drop` = enabled).
//!
//! gpui-kit's `ThemeRegistry` only reads JSON files; this module is the
//! CSS entry path for applications using this theme pack.

use std::fmt;
use std::path::Path;

use gpui_kit::component::theme::{ThemeConfig, ThemeConfigColors, ThemeMode};
use lightningcss::printer::PrinterOptions;
use lightningcss::properties::custom::{CustomProperty, Token, TokenList, TokenOrValue};
use lightningcss::properties::Property;
use lightningcss::rules::CssRule;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use lightningcss::traits::ToCss;
use lightningcss::values::color::CssColor;
use lightningcss::values::length::LengthValue;

/// Error raised while parsing a theme CSS file.
#[derive(Debug, Clone)]
pub struct ThemeCssError {
    pub message: String,
}

impl fmt::Display for ThemeCssError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "theme CSS error: {}", self.message)
    }
}

impl std::error::Error for ThemeCssError {}

fn err(message: impl Into<String>) -> ThemeCssError {
    ThemeCssError {
        message: message.into(),
    }
}

/// Reads and parses a theme file from disk.
pub fn load(path: &Path) -> Result<ThemeConfig, ThemeCssError> {
    let css = std::fs::read_to_string(path)
        .map_err(|e| err(format!("cannot read {}: {e}", path.display())))?;
    parse(&css)
}

/// Parses a theme stylesheet (`:root { --key: value; … }`) into a
/// gpui-kit [`ThemeConfig`].
pub fn parse(css: &str) -> Result<ThemeConfig, ThemeCssError> {
    let stylesheet =
        StyleSheet::parse(css, ParserOptions::default()).map_err(|e| err(format!("{e}")))?;
    let properties = root_custom_properties(&stylesheet)?;
    config_from_properties(&properties)
}

/// Collects the custom properties of the single expected `:root` rule.
fn root_custom_properties(
    stylesheet: &StyleSheet<'_>,
) -> Result<Vec<(String, String)>, ThemeCssError> {
    let mut found = false;
    let mut out = Vec::new();

    for rule in &stylesheet.rules.0 {
        let CssRule::Style(style) = rule else {
            continue;
        };
        let selector = style
            .selectors
            .to_css_string(PrinterOptions::default())
            .map_err(|e| err(format!("selector serialization failed: {e}")))?;
        if selector.trim() != ":root" {
            continue;
        }
        if found {
            return Err(err("multiple :root rules are not supported"));
        }
        found = true;
        for property in &style.declarations.declarations {
            let Property::Custom(CustomProperty { name, value }) = property else {
                return Err(err(format!(
                    "regular CSS property inside :root is not supported: {property:?}"
                )));
            };
            let key = name
                .as_ref()
                .strip_prefix("--")
                .ok_or_else(|| err(format!("custom property without -- prefix: {name:?}")))?;
            out.push((key.to_string(), value_to_string(value)?));
        }
    }

    if !found {
        return Err(err("no :root rule found"));
    }
    Ok(out)
}

/// Serializes a custom-property value into the pack's canonical string.
///
/// Only the value grammar used by theme files is accepted: hex colors,
/// identifiers, quoted strings, integers, and `px` lengths.
fn value_to_string(value: &TokenList<'_>) -> Result<String, ThemeCssError> {
    let mut out = String::new();
    for item in &value.0 {
        match item {
            TokenOrValue::Color(CssColor::RGBA(rgba)) => {
                out.push_str(&format!(
                    "#{:02x}{:02x}{:02x}{:02x}",
                    rgba.red, rgba.green, rgba.blue, rgba.alpha
                ));
            }
            TokenOrValue::Token(Token::Ident(ident)) => out.push_str(ident.as_ref()),
            TokenOrValue::Token(Token::String(string)) => out.push_str(string.as_ref()),
            TokenOrValue::Token(Token::Number { value, .. }) => {
                out.push_str(&format_number(*value));
            }
            TokenOrValue::Length(LengthValue::Px(px)) => {
                out.push_str(&format!("{}px", format_number(*px)));
            }
            other => {
                return Err(err(format!(
                    "unsupported custom property value: {other:?}"
                )));
            }
        }
    }
    if out.is_empty() {
        return Err(err("empty custom property value"));
    }
    Ok(out)
}

/// Formats a CSS number without a trailing `.0` for integers.
fn format_number(value: f32) -> String {
    if value.fract() == 0.0 && value.is_finite() {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

fn required<'a>(properties: &'a [(String, String)], key: &str) -> Result<&'a str, ThemeCssError> {
    properties
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
        .ok_or_else(|| err(format!("missing required property --{key}")))
}

/// Builds the gpui-kit configuration from the collected properties.
fn config_from_properties(properties: &[(String, String)]) -> Result<ThemeConfig, ThemeCssError> {
    let name = required(properties, "name")?;
    let mode = match required(properties, "mode")? {
        "light" => ThemeMode::Light,
        "dark" => ThemeMode::Dark,
        other => return Err(err(format!("invalid --mode value: {other:?}"))),
    };
    let radius = parse_px(required(properties, "radius")?, "--radius")?;
    let radius_lg = parse_px(required(properties, "radius-lg")?, "--radius-lg")?;
    let shadow = match required(properties, "shadow")? {
        "none" => false,
        "drop" => true,
        other => return Err(err(format!("invalid --shadow value: {other:?}"))),
    };

    let mut map = serde_json::Map::new();
    for (key, value) in properties {
        if matches!(key.as_str(), "name" | "mode" | "radius" | "radius-lg" | "shadow") {
            continue;
        }
        // `--primary-hover-background` → schema key `primary.hover.background`.
        map.insert(key.replace('-', "."), serde_json::Value::String(value.clone()));
    }

    let colors: ThemeConfigColors = serde_json::from_value(serde_json::Value::Object(map.clone()))
        .map_err(|e| err(format!("invalid color properties: {e}")))?;

    // Keys that did not land in the schema are typos; keep them visible.
    let known: std::collections::HashSet<String> = serde_json::to_value(&colors)
        .ok()
        .and_then(|value| value.as_object().map(|object| object.keys().cloned().collect()))
        .unwrap_or_default();
    for key in map.keys() {
        if !known.contains(key) {
            #[allow(clippy::print_stderr)]
            {
                eprintln!("warning: unknown theme property --{}", key.replace('.', "-"));
            }
        }
    }

    // A theme without base colors would render unstyled; require them.
    if colors.background.is_none() || colors.foreground.is_none() {
        return Err(err(
            "missing required properties --background / --foreground",
        ));
    }

    Ok(ThemeConfig {
        name: name.into(),
        mode,
        radius: Some(radius),
        radius_lg: Some(radius_lg),
        shadow: Some(shadow),
        colors,
        ..Default::default()
    })
}

fn parse_px(value: &str, key: &str) -> Result<usize, ThemeCssError> {
    let raw = value
        .strip_suffix("px")
        .ok_or_else(|| err(format!("invalid {key} value: {value:?} (expected e.g. `4px`)")))?;
    raw.parse::<usize>()
        .map_err(|_| err(format!("invalid {key} value: {value:?}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
/* OzdemirGPUIThemePack — Fluent Light Blue */
:root {
  --name: "Fluent Light Blue";
  --mode: light;
  --radius: 4px;
  --radius-lg: 8px;
  --shadow: none;
  --background: #ffffff;
  --foreground: #242424;
  --primary-background: #0066b4;
  --primary-hover-background: #0078d4;
  --link: #0066b4;
}
"#;

    #[test]
    fn parses_sample_theme() {
        let config = parse(SAMPLE).unwrap();
        assert_eq!(config.name, "Fluent Light Blue");
        assert_eq!(config.mode, ThemeMode::Light);
        assert_eq!(config.radius, Some(4));
        assert_eq!(config.radius_lg, Some(8));
        assert_eq!(config.shadow, Some(false));
        assert_eq!(config.colors.background.as_deref(), Some("#ffffffff"));
        assert_eq!(config.colors.foreground.as_deref(), Some("#242424ff"));
        assert_eq!(
            config.colors.primary.as_deref(),
            Some("#0066b4ff"),
            "6-digit hex normalizes to 8-digit"
        );
        assert_eq!(
            config.colors.primary_hover.as_deref(),
            Some("#0078d4ff")
        );
        assert_eq!(config.colors.link.as_deref(), Some("#0066b4ff"));
    }

    #[test]
    fn dark_mode_flag() {
        let css = SAMPLE.replace("--mode: light", "--mode: dark");
        let config = parse(&css).unwrap();
        assert_eq!(config.mode, ThemeMode::Dark);
    }

    #[test]
    fn shadow_drop_enables_shadows() {
        let css = SAMPLE.replace("--shadow: none", "--shadow: drop");
        let config = parse(&css).unwrap();
        assert_eq!(config.shadow, Some(true));
    }

    #[test]
    fn missing_required_property_fails() {
        let css = SAMPLE.replace("--mode: light;", "");
        assert!(parse(&css).is_err());

        let css = SAMPLE.replace("--background: #ffffff;", "");
        assert!(parse(&css).is_err());
    }

    #[test]
    fn invalid_values_fail() {
        assert!(parse(&SAMPLE.replace("--mode: light", "--mode: blue")).is_err());
        assert!(parse(&SAMPLE.replace("--radius: 4px", "--radius: 4em")).is_err());
        assert!(parse(&SAMPLE.replace("--shadow: none", "--shadow: yes")).is_err());
    }

    #[test]
    fn missing_root_rule_fails() {
        assert!(parse("body { color: red; }").is_err());
        assert!(parse("").is_err());
    }

    #[test]
    fn multiple_root_rules_fail() {
        let doubled = format!("{SAMPLE}{SAMPLE}");
        assert!(parse(&doubled).is_err());
    }

    #[test]
    fn non_root_rules_are_ignored() {
        let with_extra = format!("{SAMPLE}\nhtml {{ --ignored: 1; }}");
        let config = parse(&with_extra).unwrap();
        assert_eq!(config.name, "Fluent Light Blue");
    }

    #[test]
    fn partially_transparent_color_round_trips() {
        let css = SAMPLE.replace("--link: #0066b4", "--link: #0f6cbd66");
        let config = parse(&css).unwrap();
        assert_eq!(config.colors.link.as_deref(), Some("#0f6cbd66"));
    }
}
