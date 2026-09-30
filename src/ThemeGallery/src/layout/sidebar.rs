//! Left navigation sidebar: "General" and "Components" groups of gallery pages,
//! with the active theme shown in the footer.

use gpui_kit::assets::IconName;
use gpui_kit::component::sidebar::{Sidebar, SidebarGroup, SidebarMenuItem};
use gpui_kit::component::{ActiveTheme as _, Icon, h_flex};
use gpui_kit::{ParentElement as _, Styled as _};

use crate::app::GalleryApp;
use crate::pages::{self, PageKind};

pub fn render(
    app: &GalleryApp,
    cx: &mut gpui_kit::Context<GalleryApp>,
) -> Sidebar<SidebarGroup<SidebarMenuItem>> {
    let mut genel = SidebarGroup::new("General");
    for kind in pages::GENERAL {
        genel = genel.child(item(app, *kind, cx));
    }

    let mut bilesenler = SidebarGroup::new("Components");
    for kind in pages::COMPONENTS {
        bilesenler = bilesenler.child(item(app, *kind, cx));
    }

    let sidebar = Sidebar::new("gallery-sidebar")
        .collapsed(app.sidebar_collapsed)
        .child(genel)
        .child(bilesenler);

    if app.sidebar_collapsed {
        sidebar
    } else {
        sidebar.footer(
            h_flex()
                .gap_2()
                .items_center()
                .px_2()
                .py_1()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(Icon::new(IconName::Palette).size_3())
                .child(app.theme_label()),
        )
    }
}

fn item(
    app: &GalleryApp,
    kind: PageKind,
    cx: &mut gpui_kit::Context<GalleryApp>,
) -> SidebarMenuItem {
    SidebarMenuItem::new(kind.title())
        .icon(Icon::new(kind.icon()))
        .active(app.page == kind)
        .on_click(cx.listener(move |this, _, _, cx| {
            this.page = kind;
            cx.notify();
        }))
}
