//! Title bar: sidebar toggle and app identity on the left; the live theme
//! switcher (accent dropdown + light/dark toggle) and the search trigger on
//! the right, in the fixed 34px title bar.

use gpui_kit::assets::IconName;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::select::Select;
use gpui_kit::component::sidebar::SidebarToggleButton;
use gpui_kit::component::{ActiveTheme as _, Icon, Sizable as _, TitleBar, h_flex};
use gpui_kit::{Hsla, IntoElement, ParentElement as _, Styled as _, div, px};

use crate::app::{APP_NAME, GalleryApp};
use crate::layout;

pub fn render(app: &GalleryApp, cx: &mut gpui_kit::Context<GalleryApp>) -> TitleBar {
    TitleBar::new().child(
        h_flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .justify_between()
            .gap_2()
            .child(
                h_flex()
                    .flex_shrink_0()
                    .gap_2()
                    .items_center()
                    .child(
                        SidebarToggleButton::new()
                            .collapsed(app.sidebar_collapsed)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.sidebar_collapsed = !this.sidebar_collapsed;
                                cx.notify();
                            })),
                    )
                    .child(div().size_2().rounded_full().bg(primary_brush(app)))
                    .child(div().text_sm().font_weight(gpui_kit::FontWeight::MEDIUM).child(APP_NAME)),
            )
            .child(
                h_flex()
                    .flex_shrink_0()
                    .items_center()
                    .child(
                        h_flex()
                            .gap_1()
                            .items_center()
                            .child(Select::new(&app.accent_select).xsmall().w(px(130.)))
                            .child(mode_toggle(app, cx))
                            .child(layout::search::render(app, cx)),
                    )
                    .child(divider(cx)),
            ),
    )
}

/// A subtle vertical rule that separates the theme cluster from the window
/// control box (min/max/close) rendered by the TitleBar.
fn divider(cx: &gpui_kit::Context<GalleryApp>) -> impl IntoElement + use<> {
    gpui_kit::div()
        .w(px(1.))
        .h_4()
        .bg(cx.theme().border)
        .ml_3()
        .mr_3()
}

fn primary_brush(app: &GalleryApp) -> Hsla {
    app.accent.color().default_brush_for(app.dark).into()
}

fn mode_toggle(app: &GalleryApp, cx: &mut gpui_kit::Context<GalleryApp>) -> Button {
    Button::new("theme-mode")
        .ghost()
        .xsmall()
        .icon(Icon::new(if app.dark {
            IconName::Sun
        } else {
            IconName::Moon
        }))
        .tooltip(if app.dark { "Switch to light theme" } else { "Switch to dark theme" })
        .on_click(cx.listener(|this, _, window, cx| {
            this.dark = !this.dark;
            this.apply_theme(window, cx);
        }))
}
