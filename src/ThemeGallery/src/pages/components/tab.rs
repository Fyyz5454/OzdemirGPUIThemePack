//! Tab demos: a tab bar with selectable tabs.

use gpui_kit::component::tab::{Tab, TabBar};
use gpui_kit::component::{
    ActiveTheme as _, Selectable as _, 
};
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

const TABS: [&str; 4] = ["General", "Components", "Colors", "Theme"];

pub struct TabPage {
    selected: usize,
}

impl TabPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self { selected: 0 }
    }
}

impl Render for TabPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Tab.title(), PageKind::Tab.description(), cx);

        let selected = self.selected;
        page = page.child(
            common::section("Tab bar", cx).child(
                common::example(cx).child(
                    TabBar::new("demo-tab-bar").children(TABS.iter().enumerate().map(
                        |(ix, label)| {
                            Tab::new()
                                .label(*label)
                                .selected(ix == selected)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.selected = ix;
                                    cx.notify();
                                }))
                        },
                    )),
                ),
            ),
        );

        page = page.child(
            common::section(&format!("Active tab: {}", TABS[self.selected.min(3)]), cx).child(
                common::example(cx).child(
                    gpui_kit::div()
                        .w(px(360.))
                        .p_3()
                        .border_1()
                        .border_color(cx.theme().border)
                        .rounded(cx.theme().radius_lg)
                        .child(format!("Content of the {} tab", TABS[selected.min(3)])),
                ),
            ),
        );

        page
    }
}
