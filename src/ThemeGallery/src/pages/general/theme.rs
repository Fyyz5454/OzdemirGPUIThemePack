//! All 16 theme variants as clickable cards — the replacement for the old
//! accent radio group / dark switch column. Clicking a card applies that
//! variant live.

use gpui_kit::component::{ActiveTheme as _, h_flex};
use gpui_kit::{
    AnyView, App, AppContext as _, Hsla, InteractiveElement as _, IntoElement, ParentElement as _,
    Render, Rgba, StatefulInteractiveElement as _, Styled as _, Window, px,
};
use ozdemirgpuithemepack::fluentui::colors::ResourceDictionary;
use ozdemirgpuithemepack::fluentui::theme as fluent_theme;

use crate::pages::common;
use crate::pages::PageKind;

pub struct ThemePage;

impl ThemePage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for ThemePage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let variants: Vec<_> = fluent_theme::variants().collect();

        let mut page = common::page(
            PageKind::Theme.title(),
            PageKind::Theme.description(),
            cx,
        );

        // Light row.
        let light = variants
            .iter()
            .filter(|v| !v.dark)
            .enumerate()
            .map(|(ix, v)| card(v, ix, cx));
        page = page.child(
            common::section("Light variants", cx)
                .child(h_flex().flex_wrap().gap_3().children(light)),
        );

        // Dark row.
        let dark = variants
            .iter()
            .filter(|v| v.dark)
            .enumerate()
            .map(|(ix, v)| card(v, ix, cx));
        page = page.child(
            common::section("Dark variants", cx)
                .child(h_flex().flex_wrap().gap_3().children(dark)),
        );

        page
    }
}

fn card(
    variant: &fluent_theme::FluentVariant,
    ix: usize,
    cx: &gpui_kit::App,
) -> impl IntoElement + use<> {
    let accent = variant.accent;
    let is_dark = variant.dark;
    let ramp = accent.color();

    let dict = if is_dark {
        ResourceDictionary::dark()
    } else {
        ResourceDictionary::light()
    };
    let card_bg: Hsla = dict.solid_background_fill_color_base.into();
    let card_fg: Hsla = dict.text_fill_color_primary.into();
    let chips: [Rgba; 3] = [ramp.dark, ramp.normal, ramp.light];

    let id = gpui_kit::SharedString::from(format!(
        "variant-{}-{ix}",
        if is_dark { "dark" } else { "light" }
    ));

    h_flex()
        .id(id)
        .flex_col()
        .items_start()
        .gap_2()
        .p_3()
        .w(px(120.))
        .rounded(cx.theme().radius_lg)
        .border_1()
        .border_color(cx.theme().border)
        .bg(card_bg)
        .text_color(card_fg)
        .cursor_pointer()
        .child(
            gpui_kit::div()
                .text_size(px(12.))
                .child(fluent_theme::theme_name(accent, is_dark)),
        )
        .child(h_flex().gap_1().children(chips.map(|c| {
            let fill: Hsla = c.into();
            gpui_kit::div().size_3().rounded_sm().bg(fill)
        })))
        .on_click(move |_, window, cx| {
            fluent_theme::apply(accent, is_dark, Some(window), cx);
        })
}
