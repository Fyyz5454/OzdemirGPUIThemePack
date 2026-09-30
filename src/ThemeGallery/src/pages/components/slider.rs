//! Slider demos with live value tracking.

use gpui_kit::component::slider::{Slider, SliderEvent, SliderState};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Subscription, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct SliderPage {
    value: f32,
    slider: Entity<SliderState>,
    _subscriptions: Vec<Subscription>,
}

impl SliderPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(30.0f32)
        });

        let subscriptions = vec![cx.subscribe_in(&slider, window, |this, _, event, _, cx| {
            if let SliderEvent::Change(value) = event {
                this.value = value.start();
                cx.notify();
            }
        })];

        Self {
            value: 30.0,
            slider,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for SliderPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Slider.title(), PageKind::Slider.description(), cx);

        page = page.child(
            common::section(&format!("Value: {:.0}", self.value), cx)
                .child(Slider::new(&self.slider).w(px(280.))),
        );

        page = page.child(
            common::section("Narrow slider", cx)
                .child(Slider::new(&self.slider).w(px(160.))),
        );

        page
    }
}
