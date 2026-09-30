//! Dialog demos: trigger-opened dialogs and an alert dialog.

use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants as _};
use gpui_kit::component::dialog::{AlertDialog, Dialog, DialogButtonProps};
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct DialogPage;

impl DialogPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for DialogPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Dialog.title(), PageKind::Dialog.description(), cx);

        page = page.child(
            common::section("Temel diyalog", cx).child(
                common::example(cx).child(
                    Dialog::new(cx)
                        .trigger(Button::new("dialog-trigger").label("Open dialog").primary())
                        .title("Theme reset")
                        .content(|content, _, _| {
                            content.child(
                                gpui_kit::div()
                                    .child("Reset the theme back to the FluentLightBlue default?"),
                            )
                        })
                        .button_props(
                            DialogButtonProps::default()
                                .ok_text("Reset")
                                .cancel_text("Cancel"),
                        ),
                ),
            ),
        );

        page = page.child(
            common::section("Confirmation dialog", cx).child(
                common::example(cx).child(
                    AlertDialog::new(cx)
                        .trigger(Button::new("alert-dialog-trigger").label("Delete").danger())
                        .title("Delete file")
                        .description("This action cannot be undone. The file will be permanently deleted.")
                        .button_props(
                            DialogButtonProps::default()
                                .ok_text("Delete")
                                .ok_variant(ButtonVariant::Danger)
                                .show_cancel(true),
                        ),
                ),
            ),
        );

        page
    }
}
