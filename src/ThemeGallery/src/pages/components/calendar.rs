//! Calendar and date picker demos.

use gpui_kit::component::calendar::{Calendar, CalendarState};
use gpui_kit::component::date_picker::DatePicker;
use gpui_kit::{AnyView, ParentElement as _, Styled as _, 
    App, AppContext as _, Entity, IntoElement, Render, Window, px,
};

use crate::pages::common;
use crate::pages::PageKind;

pub struct CalendarPage {
    calendar: Entity<CalendarState>,
    date: Entity<gpui_kit::component::date_picker::DatePickerState>,
}

impl CalendarPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> Self {
        let calendar = cx.new(|cx| CalendarState::new(window, cx));
        let date = cx.new(|cx| {
            let mut state = gpui_kit::component::date_picker::DatePickerState::new(window, cx);
            state.set_year_range((2000, 2040), cx);
            state
        });
        Self { calendar, date }
    }
}

impl Render for CalendarPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(
            PageKind::Calendar.title(),
            PageKind::Calendar.description(),
            cx,
        );

        page = page.child(
            common::section("Calendar", cx)
                .child(Calendar::new(&self.calendar).w(px(280.))),
        );

        page = page.child(
            common::section("Date picker", cx)
                .child(DatePicker::new(&self.date).w(px(200.))),
        );

        page
    }
}
