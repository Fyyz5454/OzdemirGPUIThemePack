//! Text input demos: placeholder, prefix/suffix, masking, and events.

use gpui_kit::assets::IconName;
use gpui_kit::component::input::{Input, InputEvent, InputState};
use gpui_kit::component::{
    ActiveTheme as _, Icon,
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Subscription, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct InputPage {
    basic: Entity<InputState>,
    masked: Entity<InputState>,
    value: String,
    _subscriptions: Vec<Subscription>,
}

impl InputPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let basic = cx.new(|cx| InputState::new(window, cx).placeholder("Your name…"));
        let masked = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Password…")
                .masked(true)
        });

        let subscriptions = vec![cx.subscribe_in(&basic, window, |this, _, event, _, cx| {
            if matches!(event, InputEvent::Change) {
                this.value = this.basic.read(cx).value().to_string();
                cx.notify();
            }
        })];

        Self {
            basic,
            masked,
            value: String::new(),
            _subscriptions: subscriptions,
        }
    }
}

impl Render for InputPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Input.title(), PageKind::Input.description(), cx);

        page = page.child(
            common::section("Basic usage", cx).child(
                common::example(cx)
                    .child(Input::new(&self.basic).w(px(240.)))
                    .child(
                        Input::new(&self.masked)
                            .w(px(240.))
                            .prefix(Icon::new(IconName::KeyRound)),
                    )
                    .child(
                        Input::new(&self.basic)
                            .w(px(240.))
                            .disabled(true)
                            .suffix(Icon::new(IconName::Lock)),
                    ),
            ),
        );

        page = page.child(
            common::section("Value tracking", cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .text_size(px(12.))
                        .text_color(cx.theme().muted_foreground)
                        .child(format!(
                            "Length: {} — {}",
                            self.value.chars().count(),
                            if self.value.is_empty() {
                                "empty"
                            } else {
                                "filled"
                            }
                        )),
                ),
            ),
        );

        page
    }
}
