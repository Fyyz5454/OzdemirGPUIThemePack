//! Filterable combobox demos.

use gpui_kit::component::combobox::{Combobox, ComboboxState};
use gpui_kit::component::searchable_list::SearchableVec;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, App, AppContext as _, Entity, IntoElement, Render, Window, px};

use crate::pages::common;
use crate::pages::PageKind;

const CITIES: [&str; 10] = [
    "Istanbul",
    "Ankara",
    "Izmir",
    "Bursa",
    "Antalya",
    "Adana",
    "Konya",
    "Trabzon",
    "Erzurum",
    "Van",
];

pub struct ComboBoxPage {
    state: Entity<ComboboxState<SearchableVec<&'static str>>>,
}

impl ComboBoxPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let delegate = SearchableVec::new(CITIES);
        let state = cx.new(|cx| ComboboxState::new(delegate, vec![], window, cx));
        Self { state }
    }
}

impl Render for ComboBoxPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::ComboBox.title(),
            PageKind::ComboBox.description(),
            cx,
        );

        page = page.child(
            common::section("Pick a city (type to filter)", cx)
                .child(Combobox::new(&self.state).placeholder("City…").w(px(260.))),
        );

        page = page.child(
            common::section("Menu width", cx).child(
                common::example(cx).child(
                    Combobox::new(&self.state)
                        .placeholder("Wide menu…")
                        .menu_width(px(320.))
                        .w(px(260.)),
                ),
            ),
        );

        page
    }
}
