//! OTP (one-time code) input demos.

use gpui_kit::component::input::{OtpEvent, OtpInput, OtpState};
use gpui_kit::component::{
    ActiveTheme as _, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, 
    App, AppContext as _, Entity, IntoElement, Render, Subscription, Window, px,
};

use crate::pages::common;
use crate::pages::PageKind;

pub struct OtpInputPage {
    plain: Entity<OtpState>,
    masked: Entity<OtpState>,
    code: String,
    _subscriptions: Vec<Subscription>,
}

impl OtpInputPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let plain = cx.new(|cx| OtpState::new(6, window, cx));
        let masked = cx.new(|cx| OtpState::new(4, window, cx).masked(true));

        let subscriptions = vec![cx.subscribe_in(&plain, window, |this, _, event, _, cx| {
            if matches!(event, OtpEvent::Change | OtpEvent::Complete) {
                this.code = this.plain.read(cx).value().to_string();
                cx.notify();
            }
        })];

        Self {
            plain,
            masked,
            code: String::new(),
            _subscriptions: subscriptions,
        }
    }
}

impl Render for OtpInputPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::OtpInput.title(),
            PageKind::OtpInput.description(),
            cx,
        );

        page = page.child(
            common::section("6 haneli kod", cx).child(
                gpui_kit::div()
                    .w(px(240.))
                    .child(OtpInput::new(&self.plain)),
            ),
        );

        page = page.child(
            common::section("4 haneli, maskeli", cx).child(
                gpui_kit::div()
                    .w(px(180.))
                    .child(OtpInput::new(&self.masked)),
            ),
        );

        page = page.child(
            common::section("Entered value", cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .text_size(px(12.))
                        .text_color(cx.theme().muted_foreground)
                        .child(if self.code.is_empty() {
                            "—".to_string()
                        } else {
                            self.code.clone()
                        }),
                ),
            ),
        );

        page
    }
}
