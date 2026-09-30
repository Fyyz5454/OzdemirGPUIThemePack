//! Select (dropdown list) demos.

use gpui_kit::component::select::{Select, SelectEvent, SelectState};
use gpui_kit::component::{
    ActiveTheme as _, IndexPath, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, 
    App, AppContext as _, Entity, IntoElement, Render, SharedString, Subscription, Window, px,
};

use crate::pages::common;
use crate::pages::PageKind;

pub struct SelectPage {
    variant: Entity<SelectState<Vec<SharedString>>>,
    chosen: String,
    _subscriptions: Vec<Subscription>,
}

impl SelectPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let variants: Vec<SharedString> = [
            "FluentLightBlue",
            "FluentDarkBlue",
            "FluentLightTeal",
            "FluentDarkTeal",
            "FluentLightPurple",
            "FluentDarkPurple",
        ]
        .iter()
        .map(|s| (*s).into())
        .collect();

        let variant =
            cx.new(|cx| SelectState::new(variants, Some(IndexPath::new(0)), window, cx));

        let subscriptions = vec![
            cx.subscribe_in(&variant, window, |this, _, event, _, cx| {
                if let SelectEvent::Confirm(Some(value)) = event {
                    this.chosen = value.to_string();
                    cx.notify();
                }
            }),
        ];

        Self {
            variant,
            chosen: "FluentLightBlue".into(),
            _subscriptions: subscriptions,
        }
    }
}

impl Render for SelectPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Select.title(), PageKind::Select.description(), cx);

        page = page.child(
            common::section("Theme variant selection", cx)
                .child(Select::new(&self.variant).w(px(220.))),
        );

        page = page.child(
            common::section("Confirmed value", cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .text_size(px(12.))
                        .text_color(cx.theme().muted_foreground)
                        .child(self.chosen.clone()),
                ),
            ),
        );

        page
    }
}
