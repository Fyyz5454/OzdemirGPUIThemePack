//! Avatar demos: initials, icons, and sizes.

use gpui_kit::assets::IconName;
use gpui_kit::component::avatar::{Avatar, AvatarGroup};
use gpui_kit::component::{
    Icon, Sizable as _, 
};
use gpui_kit::{AnyView, ParentElement as _, App, AppContext as _, IntoElement, Render, Window};

use crate::pages::common;
use crate::pages::PageKind;

pub struct AvatarPage;

impl AvatarPage {
    pub fn view(window: &mut Window, cx: &mut App) -> AnyView {
        cx.new(|cx| Self::new(window, cx)).into()
    }

    pub fn new(_window: &mut Window, _cx: &mut gpui_kit::Context<Self>) -> Self {
        Self
    }
}

impl Render for AvatarPage {
    fn render(&mut self, _window: &mut Window, cx: &mut gpui_kit::Context<Self>) -> impl IntoElement {
        let mut page = common::page(PageKind::Avatar.title(), PageKind::Avatar.description(), cx);

        page = page.child(
            common::section("Initials", cx).child(
                common::example(cx)
                    .child(Avatar::new().name("Ozdemir"))
                    .child(Avatar::new().name("Ahmet Yilmaz"))
                    .child(Avatar::new().name("Zeynep Kaya")),
            ),
        );

        page = page.child(
            common::section("Placeholder icon and sizes", cx).child(
                common::example(cx)
                    .child(
                        Avatar::new()
                            .placeholder(Icon::new(IconName::CircleUser))
                            .xsmall(),
                    )
                    .child(
                        Avatar::new()
                            .placeholder(Icon::new(IconName::CircleUser))
                            .small(),
                    )
                    .child(Avatar::new().placeholder(Icon::new(IconName::CircleUser)))
                    .child(
                        Avatar::new()
                            .placeholder(Icon::new(IconName::CircleUser))
                            .large(),
                    ),
            ),
        );

        page = page.child(
            common::section("Avatar group", cx).child(
                common::example(cx).child(
                    AvatarGroup::new().children([
                        Avatar::new().name("Ozdemir"),
                        Avatar::new().name("Ahmet"),
                        Avatar::new().name("Zeynep"),
                        Avatar::new().name("Mehmet"),
                    ]),
                ),
            ),
        );

        page
    }
}
