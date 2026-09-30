#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeStyle {
    pub size: f32,
    pub line_height: f32,
    pub semibold: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Typography {
    pub display: TypeStyle,
    pub title_large: TypeStyle,
    pub title: TypeStyle,
    pub subtitle: TypeStyle,
    pub body_large: TypeStyle,
    pub body_strong: TypeStyle,
    pub body: TypeStyle,
    pub caption: TypeStyle,
}

impl Typography {
    pub const fn ramp() -> Self {
        Self {
            display: TypeStyle {
                size: 68.0,
                line_height: 92.0,
                semibold: true,
            },
            title_large: TypeStyle {
                size: 40.0,
                line_height: 52.0,
                semibold: true,
            },
            title: TypeStyle {
                size: 28.0,
                line_height: 36.0,
                semibold: true,
            },
            subtitle: TypeStyle {
                size: 20.0,
                line_height: 28.0,
                semibold: true,
            },
            body_large: TypeStyle {
                size: 18.0,
                line_height: 24.0,
                semibold: false,
            },
            body_strong: TypeStyle {
                size: 14.0,
                line_height: 20.0,
                semibold: true,
            },
            body: TypeStyle {
                size: 14.0,
                line_height: 20.0,
                semibold: false,
            },
            caption: TypeStyle {
                size: 12.0,
                line_height: 16.0,
                semibold: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ramp_values() {
        let t = Typography::ramp();
        assert_eq!(t.display.size, 68.0);
        assert_eq!(t.display.line_height, 92.0);
        assert!(t.display.semibold);
        assert_eq!(t.body.size, 14.0);
        assert!(!t.body.semibold);
        assert_eq!(t.caption.size, 12.0);
        assert!(!t.caption.semibold);
        assert!(t.body_strong.semibold);
    }
}
