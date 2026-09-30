//! The Fluent type ramp rendered as text specimens.

use gpui_kit::component::{ActiveTheme as _, h_flex, v_flex};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};
use ozdemirgpuithemepack::fluentui::typography::{TypeStyle, Typography};

use crate::pages::common;
use crate::pages::PageKind;

pub struct TypographyPage;

impl TypographyPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for TypographyPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let ramp = Typography::ramp();

        let mut page = common::page(
            PageKind::Typography.title(),
            PageKind::Typography.description(),
            cx,
        );

        let specimens = [
            ("Display", ramp.display),
            ("Title Large", ramp.title_large),
            ("Title", ramp.title),
            ("Subtitle", ramp.subtitle),
            ("Body Large", ramp.body_large),
            ("Body Strong", ramp.body_strong),
            ("Body", ramp.body),
            ("Caption", ramp.caption),
        ];

        let mut example = common::example(cx).flex_col().items_start().gap_3();
        for (name, style) in specimens {
            example = example.child(specimen(name, style, cx));
        }
        page = page.child(common::section("Type ramp", cx).child(example));

        page.child(
            common::section("Font family", cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .text_size(px(13.))
                        .text_color(cx.theme().muted_foreground)
                        .child("The Segoe UI Variable family is resolved at runtime and silently falls back to other fonts when unavailable."),
                ),
            ),
        )
    }
}

fn specimen(name: &'static str, style: TypeStyle, cx: &gpui_kit::App) -> impl IntoElement + use<> {
    let mut text = gpui_kit::div()
        .text_size(px(style.size))
        .line_height(px(style.line_height));
    if style.semibold {
        text = text.font_weight(gpui_kit::FontWeight::SEMIBOLD);
    }

    h_flex()
        .gap_6()
        .items_center()
        .child(
            v_flex()
                .w(px(110.))
                .gap_1()
                .child(
                    gpui_kit::div()
                        .text_size(px(12.))
                        .child(name.to_string()),
                )
                .child(
                    gpui_kit::div()
                        .text_size(px(10.))
                        .text_color(cx.theme().muted_foreground)
                        .child(format!("{} / {}", style.size, style.line_height)),
                ),
        )
        .child(text.child("Preview the Fluent theme live"))
}
