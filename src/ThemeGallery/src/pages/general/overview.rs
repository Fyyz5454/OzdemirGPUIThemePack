//! Landing page: what the theme pack is and how the gallery is organized.

use gpui_kit::component::{ActiveTheme as _};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct OverviewPage;

impl OverviewPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for OverviewPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Overview.title(),
            PageKind::Overview.description(),
            cx,
        );

        for (title, lines) in [
            (
                "What's inside?",
                &[
                    "16 theme variants: 8 accents × light/dark, built on Fluent 2 neutral ramps.",
                    "A Rust-first token library for gpui-kit (ozdemirgpuithemepack).",
                    "Generated CSS theme sheets embedded in the crate, plus the css::load loader.",
                ][..],
            ),
            (
                "Using the gallery",
                &[
                    "Pick a page from the left menu; every page shows gpui-kit controls under the active theme.",
                    "The accent dropdown and the sun/moon button in the title bar restyle the app instantly.",
                    "The magnifier button opens the page search palette.",
                ][..],
            ),
        ] {
            let mut example = common::example(cx).flex_col().items_start();
            for line in lines {
                example = example.child(
                    gpui_kit::div()
                        .text_size(px(13.))
                        .text_color(cx.theme().muted_foreground)
                        .child(format!("•  {line}")),
                );
            }
            page = page.child(common::section(title, cx).child(example));
        }

        page.child(
            common::section("About the theme", cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .text_size(px(13.))
                        .text_color(cx.theme().muted_foreground)
                        .child("Themes apply live through gpui-kit's Theme::change; see the Theme page for variant cards."),
                ),
            ),
        )
    }
}
