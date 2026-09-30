//! Shared page scaffolding: every page renders a [`page`] header inside a
//! scrollable column and groups its demos into [`section`]s with
//! [`example`] boxes (the AtlantaFX sampler layout).

use gpui_kit::component::scroll::{Scrollable, ScrollableElement as _};
use gpui_kit::component::{ActiveTheme as _, h_flex, v_flex};
use gpui_kit::{App, Div, InteractiveElement as _, ParentElement as _, Stateful, Styled as _, px};

/// The page frame: a vertically scrollable column with a title header.
pub fn page(title: &str, description: &str, cx: &App) -> Scrollable<Stateful<Div>> {
    v_flex()
        .id("page-body")
        .overflow_y_scrollbar()
        .size_full()
        .p_6()
        .gap_6()
        .child(
            v_flex()
                .gap_1()
                .child(
                    div_text(cx)
                        .text_size(px(24.))
                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                        .child(title.to_string()),
                )
                .child(
                    div_text(cx)
                        .text_size(px(13.))
                        .text_color(cx.theme().muted_foreground)
                        .child(description.to_string()),
                ),
        )
}

/// A titled section of demos below the page header.
pub fn section(title: &str, cx: &App) -> Div {
    v_flex()
        .gap_3()
        .child(div_text(cx).text_size(px(13.)).font_weight(gpui_kit::FontWeight::MEDIUM).child(title.to_string()))
}

/// A bordered box that wraps one section's demo controls.
pub fn example(cx: &App) -> Div {
    h_flex()
        .flex_wrap()
        .items_center()
        .gap_4()
        .p_4()
        .border_1()
        .border_color(cx.theme().border)
        .rounded(cx.theme().radius_lg)
}

fn div_text(cx: &App) -> Div {
    gpui_kit::div().text_color(cx.theme().foreground)
}
