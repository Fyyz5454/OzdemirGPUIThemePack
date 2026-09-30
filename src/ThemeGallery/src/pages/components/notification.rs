//! Notification demos: in-app toast notifications.

use gpui_kit::component::button::Button;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt as _;
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct NotificationPage;

impl NotificationPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for NotificationPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Notification.title(),
            PageKind::Notification.description(),
            cx,
        );

        page = page.child(
            common::section("Notification types", cx).child(
                common::example(cx)
                    .child(
                        Button::new("nt-info")
                            .label("Info")
                            .on_click(|_, window, cx| {
                                window.push_notification(
                                    Notification::info("This is an informational notification."),
                                    cx,
                                );
                            }),
                    )
                    .child(
                        Button::new("nt-success")
                            .label("Success")
                            .on_click(|_, window, cx| {
                                window.push_notification(
                                    Notification::success("The operation completed successfully."),
                                    cx,
                                );
                            }),
                    )
                    .child(
                        Button::new("nt-warning")
                            .label("Warning")
                            .on_click(|_, window, cx| {
                                window.push_notification(
                                    Notification::warning("Disk space is running low."),
                                    cx,
                                );
                            }),
                    )
                    .child(
                        Button::new("nt-error")
                            .label("Error")
                            .on_click(|_, window, cx| {
                                window.push_notification(
                                    Notification::error("Could not establish a connection."),
                                    cx,
                                );
                            }),
                    ),
            ),
        );

        page = page.child(
            common::section("With title", cx).child(
                common::example(cx).child(
                    Button::new("nt-titled")
                        .label("Titled notification")
                        .on_click(|_, window, cx| {
                            window.push_notification(
                                Notification::new()
                                    .title("Theme changed")
                                    .message("The new theme has been applied.")
                                    .with_type(gpui_kit::component::notification::NotificationType::Info),
                                cx,
                            );
                        }),
                ),
            ),
        );

        page
    }
}
