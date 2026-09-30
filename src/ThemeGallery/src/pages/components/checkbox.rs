//! Checkbox demos: labels, states, and controlled usage.

use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::{
    ActiveTheme as _, Disableable as _, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct CheckboxPage {
    checked: bool,
    preview: bool,
}

impl CheckboxPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self {
            checked: false,
            preview: true,
        }
    }
}

impl Render for CheckboxPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Checkbox.title(),
            PageKind::Checkbox.description(),
            cx,
        );

        page = page.child(
            common::section("Basic usage", cx).child(
                common::example(cx)
                    .child(
                        Checkbox::new("cb-1")
                            .label("Uncontrolled example")
                            .checked(self.checked)
                            .on_change(cx.listener(|this, value: &bool, _, cx| {
                                this.checked = *value;
                                cx.notify();
                            })),
                    )
                    .child(Checkbox::new("cb-2").label("Checked by default").checked(true))
                    .child(
                        Checkbox::new("cb-3")
                            .label("Disabled")
                            .checked(true)
                            .disabled(true),
                    )
                    .child(Checkbox::new("cb-4").label("Disabled, unchecked").disabled(true)),
            ),
        );

        let state = if self.preview { "on" } else { "off" };
        page = page.child(
            common::section("Controlled usage", cx).child(
                common::example(cx)
                    .child(
                        Checkbox::new("cb-preview")
                            .label("Live preview")
                            .checked(self.preview)
                            .on_change(cx.listener(|this, value: &bool, _, cx| {
                                this.preview = *value;
                                cx.notify();
                            })),
                    )
                    .child(
                        gpui_kit::div()
                            .text_size(px(12.))
                            .text_color(cx.theme().muted_foreground)
                            .child(format!("Preview state: {state}")),
                    ),
            ),
        );

        page
    }
}
