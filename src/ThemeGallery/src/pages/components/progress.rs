//! Progress bar and progress circle demos, driven by a slider.

use gpui_kit::component::progress::{Progress, ProgressCircle};
use gpui_kit::component::slider::{Slider, SliderEvent, SliderState};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Subscription, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

pub struct ProgressPage {
    value: f32,
    slider: Entity<SliderState>,
    _subscriptions: Vec<Subscription>,
}

impl ProgressPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let slider = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(45.0f32)
        });

        let subscriptions = vec![cx.subscribe_in(&slider, window, |this, _, event, _, cx| {
            if let SliderEvent::Change(value) = event {
                this.value = value.start();
                cx.notify();
            }
        })];

        Self {
            value: 45.0,
            slider,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for ProgressPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let ratio = (self.value / 100.0).clamp(0.0, 1.0);
        let mut page = common::page(
            PageKind::Progress.title(),
            PageKind::Progress.description(),
            cx,
        );

        page = page.child(
            common::section(&format!("Value: {:.0}%", self.value), cx).child(
                common::example(cx)
                    .child(Progress::new("progress-demo").value(ratio).w(px(320.)))
                    .child(
                        ProgressCircle::new("progress-circle")
                            .value(ratio)
                            .size_10(),
                    ),
            ),
        );

        page = page.child(
            common::section("Driver", cx)
                .child(Slider::new(&self.slider).w(px(320.))),
        );

        page
    }
}
