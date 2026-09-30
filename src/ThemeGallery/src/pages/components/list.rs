//! List demos with a small custom delegate.

use gpui_kit::assets::IconName;
use gpui_kit::component::list::{List, ListDelegate, ListItem, ListState};
use gpui_kit::component::{Icon, IndexPath};
use gpui_kit::{
    AnyView, App, AppContext as _, Entity, IntoElement, ParentElement as _, Render, SharedString,
    Styled as _, Window, px,
};

use crate::pages::common;
use crate::pages::PageKind;

struct CityDelegate {
    items: Vec<SharedString>,
    selected: Option<usize>,
}

impl ListDelegate for CityDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &gpui_kit::App) -> usize {
        self.items.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut gpui_kit::Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let row = ix.row;
        let label = self.items.get(row)?.clone();
        Some(
            ListItem::new(("city", row))
                .selected(self.selected == Some(row))
                .child(label),
        )
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        _cx: &mut gpui_kit::Context<ListState<Self>>,
    ) {
        self.selected = ix.map(|ix| ix.row);
    }
}

pub struct ListPage {
    state: Entity<ListState<CityDelegate>>,
}

impl ListPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let delegate = CityDelegate {
            items: [
                "Istanbul",
                "Ankara",
                "Izmir",
                "Bursa",
                "Antalya",
                "Adana",
                "Konya",
                "Trabzon",
            ]
            .iter()
            .map(|s| (*s).into())
            .collect(),
            selected: Some(0),
        };
        let state = cx.new(|cx| ListState::new(delegate, window, cx));
        Self { state }
    }
}

impl Render for ListPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::List.title(), PageKind::List.description(), cx);

        page = page.child(
            common::section("City list", cx).child(
                gpui_kit::div()
                    .h(px(260.))
                    .w(px(280.))
                    .child(List::new(&self.state)),
            ),
        );

        page = page.child(
            common::section("List item samples", cx).child(
                common::example(cx)
                    .child(
                        ListItem::new("li-plain").child("Plain item"),
                    )
                    .child(
                        ListItem::new("li-selected")
                            .selected(true)
                            .child("Selected item"),
                    )
                    .child(
                        ListItem::new("li-icon")
                            .child(Icon::new(IconName::MapPin))
                            .child("Item with icon"),
                    ),
            ),
        );

        page
    }
}
