use gpui_kit::Rgba;

/// Fluent 2 token constants are kept in ARGB (0xAARRGGBB) order.
fn c(argb: u32) -> Rgba {
    let a = argb >> 24;
    let rgb = argb & 0x00ff_ffff;
    gpui_kit::rgba((rgb << 8) | a)
}

/// Converts an Rgba into the `#RRGGBBAA` string accepted by gpui-kit theme files.
pub fn hex_string(color: Rgba) -> String {
    let bytes = [
        (color.r * 255.0).round() as u8,
        (color.g * 255.0).round() as u8,
        (color.b * 255.0).round() as u8,
        (color.a * 255.0).round() as u8,
    ];
    format!(
        "#{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3]
    )
}

/// Replaces the alpha channel of the given color.
pub fn with_alpha(color: Rgba, alpha: f32) -> Rgba {
    Rgba {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceDictionary {
    pub text_fill_color_primary: Rgba,
    pub text_fill_color_secondary: Rgba,
    pub text_fill_color_tertiary: Rgba,
    pub text_fill_color_disabled: Rgba,
    pub text_fill_color_inverse: Rgba,
    pub accent_text_fill_color_disabled: Rgba,
    pub text_on_accent_fill_color_selected_text: Rgba,
    pub text_on_accent_fill_color_primary: Rgba,
    pub text_on_accent_fill_color_secondary: Rgba,
    pub text_on_accent_fill_color_disabled: Rgba,
    pub control_fill_color_default: Rgba,
    pub control_fill_color_secondary: Rgba,
    pub control_fill_color_tertiary: Rgba,
    pub control_fill_color_quarternary: Rgba,
    pub control_fill_color_disabled: Rgba,
    pub control_fill_color_transparent: Rgba,
    pub control_fill_color_input_active: Rgba,
    pub control_strong_fill_color_default: Rgba,
    pub control_strong_fill_color_disabled: Rgba,
    pub control_solid_fill_color_default: Rgba,
    pub subtle_fill_color_transparent: Rgba,
    pub subtle_fill_color_secondary: Rgba,
    pub subtle_fill_color_tertiary: Rgba,
    pub subtle_fill_color_disabled: Rgba,
    pub control_alt_fill_color_transparent: Rgba,
    pub control_alt_fill_color_secondary: Rgba,
    pub control_alt_fill_color_tertiary: Rgba,
    pub control_alt_fill_color_quarternary: Rgba,
    pub control_alt_fill_color_disabled: Rgba,
    pub control_on_image_fill_color_default: Rgba,
    pub control_on_image_fill_color_secondary: Rgba,
    pub control_on_image_fill_color_tertiary: Rgba,
    pub control_on_image_fill_color_disabled: Rgba,
    pub accent_fill_color_disabled: Rgba,
    pub control_stroke_color_default: Rgba,
    pub control_stroke_color_secondary: Rgba,
    pub control_stroke_color_on_accent_default: Rgba,
    pub control_stroke_color_on_accent_secondary: Rgba,
    pub control_stroke_color_on_accent_tertiary: Rgba,
    pub control_stroke_color_on_accent_disabled: Rgba,
    pub control_stroke_color_for_strong_fill_when_on_image: Rgba,
    pub card_stroke_color_default: Rgba,
    pub card_stroke_color_default_solid: Rgba,
    pub control_strong_stroke_color_default: Rgba,
    pub control_strong_stroke_color_disabled: Rgba,
    pub surface_stroke_color_default: Rgba,
    pub surface_stroke_color_flyout: Rgba,
    pub surface_stroke_color_inverse: Rgba,
    pub divider_stroke_color_default: Rgba,
    pub focus_stroke_color_outer: Rgba,
    pub focus_stroke_color_inner: Rgba,
    pub card_background_fill_color_default: Rgba,
    pub card_background_fill_color_secondary: Rgba,
    pub card_background_fill_color_tertiary: Rgba,
    pub smoke_fill_color_default: Rgba,
    pub layer_fill_color_default: Rgba,
    pub layer_fill_color_alt: Rgba,
    pub layer_on_acrylic_fill_color_default: Rgba,
    pub layer_on_accent_acrylic_fill_color_default: Rgba,
    pub layer_on_mica_base_alt_fill_color_default: Rgba,
    pub layer_on_mica_base_alt_fill_color_secondary: Rgba,
    pub layer_on_mica_base_alt_fill_color_tertiary: Rgba,
    pub layer_on_mica_base_alt_fill_color_transparent: Rgba,
    pub solid_background_fill_color_base: Rgba,
    pub solid_background_fill_color_secondary: Rgba,
    pub solid_background_fill_color_tertiary: Rgba,
    pub solid_background_fill_color_quarternary: Rgba,
    pub solid_background_fill_color_quinary: Rgba,
    pub solid_background_fill_color_senary: Rgba,
    pub solid_background_fill_color_transparent: Rgba,
    pub solid_background_fill_color_base_alt: Rgba,
    pub system_fill_color_success: Rgba,
    pub system_fill_color_caution: Rgba,
    pub system_fill_color_critical: Rgba,
    pub system_fill_color_neutral: Rgba,
    pub system_fill_color_solid_neutral: Rgba,
    pub system_fill_color_attention_background: Rgba,
    pub system_fill_color_success_background: Rgba,
    pub system_fill_color_caution_background: Rgba,
    pub system_fill_color_critical_background: Rgba,
    pub system_fill_color_neutral_background: Rgba,
    pub system_fill_color_solid_attention_background: Rgba,
    pub system_fill_color_solid_neutral_background: Rgba,
}

impl ResourceDictionary {
    pub fn light() -> Self {
        Self {
            text_fill_color_primary: c(0xE4000000),
            text_fill_color_secondary: c(0x9E000000),
            text_fill_color_tertiary: c(0x72000000),
            text_fill_color_disabled: c(0x5C000000),
            text_fill_color_inverse: c(0xFFFFFFFF),
            accent_text_fill_color_disabled: c(0x5C000000),
            text_on_accent_fill_color_selected_text: c(0xFFFFFFFF),
            text_on_accent_fill_color_primary: c(0xFFFFFFFF),
            text_on_accent_fill_color_secondary: c(0xB3FFFFFF),
            text_on_accent_fill_color_disabled: c(0xFFFFFFFF),
            control_fill_color_default: c(0xB3FFFFFF),
            control_fill_color_secondary: c(0x80F9F9F9),
            control_fill_color_tertiary: c(0x4DF9F9F9),
            control_fill_color_quarternary: c(0xC2F3F3F3),
            control_fill_color_disabled: c(0x4DF9F9F9),
            control_fill_color_transparent: c(0x00FFFFFF),
            control_fill_color_input_active: c(0xFFFFFFFF),
            control_strong_fill_color_default: c(0x72000000),
            control_strong_fill_color_disabled: c(0x51000000),
            control_solid_fill_color_default: c(0xFFFFFFFF),
            subtle_fill_color_transparent: c(0x00FFFFFF),
            subtle_fill_color_secondary: c(0x09000000),
            subtle_fill_color_tertiary: c(0x06000000),
            subtle_fill_color_disabled: c(0x00FFFFFF),
            control_alt_fill_color_transparent: c(0x00FFFFFF),
            control_alt_fill_color_secondary: c(0x06000000),
            control_alt_fill_color_tertiary: c(0x0F000000),
            control_alt_fill_color_quarternary: c(0x18000000),
            control_alt_fill_color_disabled: c(0x00FFFFFF),
            control_on_image_fill_color_default: c(0xC9FFFFFF),
            control_on_image_fill_color_secondary: c(0xFFF3F3F3),
            control_on_image_fill_color_tertiary: c(0xFFEBEBEB),
            control_on_image_fill_color_disabled: c(0x00FFFFFF),
            accent_fill_color_disabled: c(0x37000000),
            control_stroke_color_default: c(0x0F000000),
            control_stroke_color_secondary: c(0x29000000),
            control_stroke_color_on_accent_default: c(0x14FFFFFF),
            control_stroke_color_on_accent_secondary: c(0x66000000),
            control_stroke_color_on_accent_tertiary: c(0x37000000),
            control_stroke_color_on_accent_disabled: c(0x0F000000),
            control_stroke_color_for_strong_fill_when_on_image: c(0x59FFFFFF),
            card_stroke_color_default: c(0x0F000000),
            card_stroke_color_default_solid: c(0xFFEBEBEB),
            control_strong_stroke_color_default: c(0x72000000),
            control_strong_stroke_color_disabled: c(0x37000000),
            surface_stroke_color_default: c(0x66757575),
            surface_stroke_color_flyout: c(0x0F000000),
            surface_stroke_color_inverse: c(0x15FFFFFF),
            divider_stroke_color_default: c(0x0F000000),
            focus_stroke_color_outer: c(0xE4000000),
            focus_stroke_color_inner: c(0xB3FFFFFF),
            card_background_fill_color_default: c(0xB3FFFFFF),
            card_background_fill_color_secondary: c(0x80F6F6F6),
            card_background_fill_color_tertiary: c(0xFFFFFFFF),
            smoke_fill_color_default: c(0x4D000000),
            layer_fill_color_default: c(0x80FFFFFF),
            layer_fill_color_alt: c(0xFFFFFFFF),
            layer_on_acrylic_fill_color_default: c(0x40FFFFFF),
            layer_on_accent_acrylic_fill_color_default: c(0x40FFFFFF),
            layer_on_mica_base_alt_fill_color_default: c(0xB3FFFFFF),
            layer_on_mica_base_alt_fill_color_secondary: c(0x0A000000),
            layer_on_mica_base_alt_fill_color_tertiary: c(0xFFF9F9F9),
            layer_on_mica_base_alt_fill_color_transparent: c(0x00000000),
            solid_background_fill_color_base: c(0xFFF3F3F3),
            solid_background_fill_color_secondary: c(0xFFEEEEEE),
            solid_background_fill_color_tertiary: c(0xFFF9F9F9),
            solid_background_fill_color_quarternary: c(0xFFFFFFFF),
            solid_background_fill_color_quinary: c(0xFFFDFDFD),
            solid_background_fill_color_senary: c(0xFFFFFFFF),
            solid_background_fill_color_transparent: c(0x00F3F3F3),
            solid_background_fill_color_base_alt: c(0xFFDADADA),
            system_fill_color_success: c(0xFF0F7B0F),
            system_fill_color_caution: c(0xFF9D5D00),
            system_fill_color_critical: c(0xFFC42B1C),
            system_fill_color_neutral: c(0x72000000),
            system_fill_color_solid_neutral: c(0xFF8A8A8A),
            system_fill_color_attention_background: c(0x80F6F6F6),
            system_fill_color_success_background: c(0xFFDFF6DD),
            system_fill_color_caution_background: c(0xFFFFF4CE),
            system_fill_color_critical_background: c(0xFFFDE7E9),
            system_fill_color_neutral_background: c(0x06000000),
            system_fill_color_solid_attention_background: c(0xFFF7F7F7),
            system_fill_color_solid_neutral_background: c(0xFFF3F3F3),
        }
    }

    pub fn dark() -> Self {
        Self {
            text_fill_color_primary: c(0xFFFFFFFF),
            text_fill_color_secondary: c(0xC5FFFFFF),
            text_fill_color_tertiary: c(0x87FFFFFF),
            text_fill_color_disabled: c(0x5DFFFFFF),
            text_fill_color_inverse: c(0xE4000000),
            accent_text_fill_color_disabled: c(0x5DFFFFFF),
            text_on_accent_fill_color_selected_text: c(0xFFFFFFFF),
            text_on_accent_fill_color_primary: c(0xFF000000),
            text_on_accent_fill_color_secondary: c(0x80000000),
            text_on_accent_fill_color_disabled: c(0x87FFFFFF),
            control_fill_color_default: c(0x0FFFFFFF),
            control_fill_color_secondary: c(0x15FFFFFF),
            control_fill_color_tertiary: c(0x08FFFFFF),
            control_fill_color_quarternary: c(0x0FFFFFFF),
            control_fill_color_disabled: c(0x0BFFFFFF),
            control_fill_color_transparent: c(0x00FFFFFF),
            control_fill_color_input_active: c(0xB31E1E1E),
            control_strong_fill_color_default: c(0x8BFFFFFF),
            control_strong_fill_color_disabled: c(0x3FFFFFFF),
            control_solid_fill_color_default: c(0xFF454545),
            subtle_fill_color_transparent: c(0x00FFFFFF),
            subtle_fill_color_secondary: c(0x0FFFFFFF),
            subtle_fill_color_tertiary: c(0x0AFFFFFF),
            subtle_fill_color_disabled: c(0x00FFFFFF),
            control_alt_fill_color_transparent: c(0x00FFFFFF),
            control_alt_fill_color_secondary: c(0x19000000),
            control_alt_fill_color_tertiary: c(0x0BFFFFFF),
            control_alt_fill_color_quarternary: c(0x12FFFFFF),
            control_alt_fill_color_disabled: c(0x00FFFFFF),
            control_on_image_fill_color_default: c(0xB31C1C1C),
            control_on_image_fill_color_secondary: c(0xFF1A1A1A),
            control_on_image_fill_color_tertiary: c(0xFF131313),
            control_on_image_fill_color_disabled: c(0xFF1E1E1E),
            accent_fill_color_disabled: c(0x28FFFFFF),
            control_stroke_color_default: c(0x12FFFFFF),
            control_stroke_color_secondary: c(0x18FFFFFF),
            control_stroke_color_on_accent_default: c(0x14FFFFFF),
            control_stroke_color_on_accent_secondary: c(0x23000000),
            control_stroke_color_on_accent_tertiary: c(0x37000000),
            control_stroke_color_on_accent_disabled: c(0x33000000),
            control_stroke_color_for_strong_fill_when_on_image: c(0x6B000000),
            card_stroke_color_default: c(0x19000000),
            card_stroke_color_default_solid: c(0xFF1C1C1C),
            control_strong_stroke_color_default: c(0x8BFFFFFF),
            control_strong_stroke_color_disabled: c(0x28FFFFFF),
            surface_stroke_color_default: c(0x66757575),
            surface_stroke_color_flyout: c(0x33000000),
            surface_stroke_color_inverse: c(0x0F000000),
            divider_stroke_color_default: c(0x15FFFFFF),
            focus_stroke_color_outer: c(0xFFFFFFFF),
            focus_stroke_color_inner: c(0xB3000000),
            card_background_fill_color_default: c(0x0DFFFFFF),
            card_background_fill_color_secondary: c(0x08FFFFFF),
            card_background_fill_color_tertiary: c(0x12FFFFFF),
            smoke_fill_color_default: c(0x4D000000),
            layer_fill_color_default: c(0x4C3A3A3A),
            layer_fill_color_alt: c(0x0DFFFFFF),
            layer_on_acrylic_fill_color_default: c(0x09FFFFFF),
            layer_on_accent_acrylic_fill_color_default: c(0x09FFFFFF),
            layer_on_mica_base_alt_fill_color_default: c(0x733A3A3A),
            layer_on_mica_base_alt_fill_color_secondary: c(0x0FFFFFFF),
            layer_on_mica_base_alt_fill_color_tertiary: c(0xFF2C2C2C),
            layer_on_mica_base_alt_fill_color_transparent: c(0x00FFFFFF),
            solid_background_fill_color_base: c(0xFF202020),
            solid_background_fill_color_secondary: c(0xFF1C1C1C),
            solid_background_fill_color_tertiary: c(0xFF282828),
            solid_background_fill_color_quarternary: c(0xFF2C2C2C),
            solid_background_fill_color_quinary: c(0xFF333333),
            solid_background_fill_color_senary: c(0xFF373737),
            solid_background_fill_color_transparent: c(0x00202020),
            solid_background_fill_color_base_alt: c(0xFF0A0A0A),
            system_fill_color_success: c(0xFF6CCB5F),
            system_fill_color_caution: c(0xFFFCE100),
            system_fill_color_critical: c(0xFFFF99A4),
            system_fill_color_neutral: c(0x8BFFFFFF),
            system_fill_color_solid_neutral: c(0xFF9D9D9D),
            system_fill_color_attention_background: c(0x08FFFFFF),
            system_fill_color_success_background: c(0xFF393D1B),
            system_fill_color_caution_background: c(0xFF433519),
            system_fill_color_critical_background: c(0xFF442726),
            system_fill_color_neutral_background: c(0x08FFFFFF),
            system_fill_color_solid_attention_background: c(0xFF2E2E2E),
            system_fill_color_solid_neutral_background: c(0xFF2E2E2E),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AccentColor {
    pub darkest: Rgba,
    pub darker: Rgba,
    pub dark: Rgba,
    pub normal: Rgba,
    pub light: Rgba,
    pub lighter: Rgba,
    pub lightest: Rgba,
}

impl AccentColor {
    pub fn swatch(
        darkest: Rgba,
        darker: Rgba,
        dark: Rgba,
        normal: Rgba,
        light: Rgba,
        lighter: Rgba,
        lightest: Rgba,
    ) -> Self {
        Self {
            darkest,
            darker,
            dark,
            normal,
            light,
            lighter,
            lightest,
        }
    }

    pub fn default_brush_for(&self, dark_mode: bool) -> Rgba {
        if dark_mode {
            self.lighter
        } else {
            self.dark
        }
    }
}

/// Accent color palettes supported by fluent_ui.
#[allow(dead_code)]
pub mod palette {
    use super::{c, AccentColor, Rgba};

    pub fn blue() -> AccentColor {
        AccentColor::swatch(
            c(0xFF004A83),
            c(0xFF005494),
            c(0xFF0066B4),
            c(0xFF0078D4),
            c(0xFF268CDA),
            c(0xFF4CA0E0),
            c(0xFF60ABE4),
        )
    }

    pub fn red() -> AccentColor {
        AccentColor::swatch(
            c(0xFF8F0A15),
            c(0xFFA20B18),
            c(0xFFB90D1C),
            c(0xFFE81123),
            c(0xFFEC404F),
            c(0xFFEE5865),
            c(0xFFF06B76),
        )
    }

    pub fn green() -> AccentColor {
        AccentColor::swatch(
            c(0xFF094C09),
            c(0xFF0C5D0C),
            c(0xFF0E6F0E),
            c(0xFF107C10),
            c(0xFF278927),
            c(0xFF4B9C4B),
            c(0xFF6AAD6A),
        )
    }

    pub fn teal() -> AccentColor {
        AccentColor::swatch(
            c(0xFF006E5B),
            c(0xFF007C67),
            c(0xFF00977D),
            c(0xFF00B294),
            c(0xFF26BDA4),
            c(0xFF4CC9B4),
            c(0xFF60CFBC),
        )
    }

    pub fn purple() -> AccentColor {
        AccentColor::swatch(
            c(0xFF472F68),
            c(0xFF513576),
            c(0xFF644293),
            c(0xFF744DA9),
            c(0xFF8664B4),
            c(0xFF9D82C2),
            c(0xFFA890C9),
        )
    }

    pub fn magenta() -> AccentColor {
        AccentColor::swatch(
            c(0xFF6F0061),
            c(0xFF7E006E),
            c(0xFF90007E),
            c(0xFFB4009E),
            c(0xFFC333B1),
            c(0xFFCA4CBB),
            c(0xFFD060C2),
        )
    }

    pub fn orange() -> AccentColor {
        AccentColor::swatch(
            c(0xFF993D07),
            c(0xFFAC4508),
            c(0xFFD1540A),
            c(0xFFF7630C),
            c(0xFFF87A30),
            c(0xFFF99154),
            c(0xFFFA9E68),
        )
    }

    pub fn yellow() -> AccentColor {
        AccentColor::swatch(
            c(0xFFF9A825),
            c(0xFFFBC02D),
            c(0xFFFDD835),
            c(0xFFFFEB3B),
            c(0xFFFFEE58),
            c(0xFFFFF176),
            c(0xFFFFF59D),
        )
    }

    pub fn warning_primary() -> Rgba {
        c(0xFFD83B01)
    }

    pub fn error_primary() -> Rgba {
        c(0xFFA80000)
    }

    pub fn success_primary() -> Rgba {
        c(0xFF107C10)
    }

    pub fn grey(shade: u32) -> Rgba {
        match shade {
            220 => c(0xFF11100F),
            210 => c(0xFF161514),
            200 => c(0xFF1B1A19),
            190 => c(0xFF201F1E),
            180 => c(0xFF252423),
            170 => c(0xFF292827),
            160 => c(0xFF323130),
            150 => c(0xFF3B3A39),
            140 => c(0xFF484644),
            130 => c(0xFF605E5C),
            120 => c(0xFF797775),
            110 => c(0xFF8A8886),
            100 => c(0xFF979593),
            90 => c(0xFFA19F9D),
            80 => c(0xFFB3B0AD),
            70 => c(0xFFBEBBB8),
            60 => c(0xFFC8C6C4),
            50 => c(0xFFD2D0CE),
            40 => c(0xFFE1DFDD),
            30 => c(0xFFEDEBE9),
            20 => c(0xFFF3F2F1),
            _ => c(0xFFFAF9F8),
        }
    }
}

/// Selectable accent colors; each one yields a Fluent theme in 2 brightness variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accent {
    Blue,
    Red,
    Green,
    Teal,
    Purple,
    Magenta,
    Orange,
    Yellow,
}

impl Accent {
    pub const ALL: [Accent; 8] = [
        Accent::Blue,
        Accent::Red,
        Accent::Green,
        Accent::Teal,
        Accent::Purple,
        Accent::Magenta,
        Accent::Orange,
        Accent::Yellow,
    ];

    pub fn color(self) -> AccentColor {
        match self {
            Accent::Blue => palette::blue(),
            Accent::Red => palette::red(),
            Accent::Green => palette::green(),
            Accent::Teal => palette::teal(),
            Accent::Purple => palette::purple(),
            Accent::Magenta => palette::magenta(),
            Accent::Orange => palette::orange(),
            Accent::Yellow => palette::yellow(),
        }
    }

    /// Display name inside the theme registry.
    pub fn name(self) -> &'static str {
        match self {
            Accent::Blue => "Blue",
            Accent::Red => "Red",
            Accent::Green => "Green",
            Accent::Teal => "Teal",
            Accent::Purple => "Purple",
            Accent::Magenta => "Magenta",
            Accent::Orange => "Orange",
            Accent::Yellow => "Yellow",
        }
    }

    /// Turkish label shown in the UI.
    pub fn label(self) -> &'static str {
        match self {
            Accent::Blue => "Mavi",
            Accent::Red => "Kırmızı",
            Accent::Green => "Yeşil",
            Accent::Teal => "Turkuaz",
            Accent::Purple => "Mor",
            Accent::Magenta => "Macenta",
            Accent::Orange => "Turuncu",
            Accent::Yellow => "Sarı",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_base_tokens() {
        let r = ResourceDictionary::light();
        assert_eq!(
            r.solid_background_fill_color_base,
            gpui_kit::rgba(0xf3f3f3ff)
        );
        assert_eq!(r.system_fill_color_critical, gpui_kit::rgba(0xc42b1cff));
    }

    #[test]
    fn dark_base_tokens() {
        let r = ResourceDictionary::dark();
        assert_eq!(
            r.solid_background_fill_color_base,
            gpui_kit::rgba(0x202020ff)
        );
        assert_eq!(
            r.solid_background_fill_color_secondary,
            gpui_kit::rgba(0x1c1c1cff)
        );
    }

    #[test]
    fn semi_transparent_tokens() {
        let r = ResourceDictionary::light();
        // ARGB 0x09000000 -> RGBA #00000009
        assert_eq!(r.subtle_fill_color_secondary, gpui_kit::rgba(0x00000009));
        // ARGB 0xb3ffffff -> RGBA #ffffffb3
        assert_eq!(r.control_fill_color_default, gpui_kit::rgba(0xffffffb3));
    }

    #[test]
    fn accent_swatches() {
        assert_eq!(palette::blue().normal, gpui_kit::rgba(0x0078d4ff));
        assert_eq!(palette::blue().default_brush_for(false), palette::blue().dark);
        assert_eq!(
            palette::blue().default_brush_for(true),
            palette::blue().lighter
        );
    }

    #[test]
    fn grey_ramp_bounds() {
        assert_eq!(palette::grey(160), gpui_kit::rgba(0x323130ff));
        assert_eq!(palette::grey(10), gpui_kit::rgba(0xfaf9f8ff));
    }

    #[test]
    fn hex_string_roundtrip() {
        for color in [
            ResourceDictionary::light().solid_background_fill_color_base,
            ResourceDictionary::light().subtle_fill_color_secondary,
            ResourceDictionary::dark().focus_stroke_color_inner,
            palette::orange().normal,
        ] {
            let hex = hex_string(color);
            assert!(hex.starts_with('#'));
            assert_eq!(hex.len(), 9);
            let parsed: Rgba = hex.as_str().try_into().unwrap();
            assert_eq!(parsed, color);
        }
    }

    #[test]
    fn accent_enumeration() {
        assert_eq!(Accent::ALL.len(), 8);
        assert_eq!(Accent::Blue.name(), "Blue");
        assert_eq!(Accent::Yellow.label(), "Sarı");
        assert!(Accent::ALL.iter().all(|a| a.color().normal.a > 0.99));
    }
}
