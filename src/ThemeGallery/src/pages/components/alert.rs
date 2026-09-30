//! Alert demos: info, success, warning, error, and banner mode.

use gpui_kit::component::alert::Alert;
use gpui_kit::component::v_flex;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct AlertPage;

impl AlertPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for AlertPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Alert.title(), PageKind::Alert.description(), cx);

        page = page.child(
            common::section("Variants", cx).child(
                v_flex()
                    .gap_3()
                    .w(px(420.))
                    .child(Alert::info("alert-info", "This is an informational message."))
                    .child(Alert::success("alert-success", "Changes were saved."))
                    .child(Alert::warning("alert-warning", "This action cannot be undone."))
                    .child(Alert::error("alert-error", "Something went wrong.")),
            ),
        );

        page = page.child(
            common::section("Titled and banner", cx).child(
                v_flex()
                    .gap_3()
                    .w(px(420.))
                    .child(
                        Alert::info("alert-titled", "Details are in the message.")
                            .title("Info")
                            .on_close(|_, _, _| {}),
                    )
                    .child(
                        Alert::warning("alert-banner", "A maintenance window is approaching.")
                            .banner(),
                    ),
            ),
        );

        page
    }
}
