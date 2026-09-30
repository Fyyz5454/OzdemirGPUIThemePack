//! Stepper demos: horizontal and vertical step flows.

use gpui_kit::assets::IconName;
use gpui_kit::component::stepper::{Stepper, StepperItem};
use gpui_kit::component::Icon;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct StepperPage {
    current: usize,
}

impl StepperPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self { current: 1 }
    }
}

impl Render for StepperPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Stepper.title(),
            PageKind::Stepper.description(),
            cx,
        );

        let current = self.current;

        page = page.child(
            common::section("Horizontal steps", cx).child(
                gpui_kit::div().w(px(420.)).child(
                    Stepper::new("stepper-h")
                        .selected_index(current)
                        .on_click(cx.listener(move |this, ix: &usize, _, cx| {
                            this.current = (*ix).min(2);
                            cx.notify();
                        }))
                        .items([
                            StepperItem::new()
                                .icon(Icon::new(IconName::Check))
                                .child("Pick a theme"),
                            StepperItem::new().child("Browse components"),
                            StepperItem::new().child("Save"),
                        ]),
                ),
            ),
        );

        page = page.child(
            common::section("Vertical steps", cx).child(
                gpui_kit::div().w(px(280.)).child(
                    Stepper::new("stepper-v")
                        .vertical()
                        .selected_index(current)
                        .on_click(cx.listener(move |this, ix: &usize, _, cx| {
                            this.current = (*ix).min(2);
                            cx.notify();
                        }))
                        .items([
                            StepperItem::new().child("Download"),
                            StepperItem::new().child("Install"),
                            StepperItem::new().child("Ready"),
                        ]),
                ),
            ),
        );

        page
    }
}
