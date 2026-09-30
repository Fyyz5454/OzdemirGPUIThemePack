# GpuiKit — bu kod tabanında bileşen kalıpları

> Bu belge [.kilo/skills/GpuiKit/SKILL.md](../.kilo/skills/GpuiKit/SKILL.md) dosyasının Türkçe aynasıdır.

Referans: https://gpui-kit.com/ (crate `gpui-kit = "0.6"`; iç modüller `gpui_component` üzerinden yeniden ihraç edilir). Çalışan örnekler `src/ThemeGallery/src/main.rs` içindedir.

## 1. Uygulama önyüklemesi

```rust
gpui_kit::application()
    .with_assets(gpui_kit::assets::Assets)
    .run(|cx| {
        gpui_kit::init(cx);               // İLK satır olmalı
        // tema kurulumu, sonra:
        cx.open_window(options, |window, cx| {
            let view: Entity<T> = cx.new(|cx| T::new(window, cx));
            cx.new(|cx| Root::new(AnyView::from(view), window, cx))
        })
        .expect("pencere açılamadı");
    });
```

- `open_window`'a dönen görünüm `Root` ile sarılmalıdır.
- `render` içinde kök `div`'in sonuna katmanları ekleyin:
  `Root::render_dialog_layer(window, cx)`, `Root::render_sheet_layer(...)`, `Root::render_notification_layer(...)`.

## 2. Pencere ayarları ve istemci taraflı başlık çubuğu

```rust
let mut options = TitleBar::window_options();
options.window_bounds = Some(WindowBounds::Windowed(Bounds::centered(None, size(px(800.), px(600.)), cx)));
options.window_decorations = Some(WindowDecorations::Client); // WM başlık çubuğu yok (X11'de compositor ister; yoksa otomatik geri düşme)
if let Some(tb) = options.titlebar.as_mut() { tb.title = Some(APP_NAME.into()); }
```

`TitleBar`, Linux'ta kendi küçült/büyüt/kapat düğmelerini çizer; sürükleme ve çift-tık büyütmeyi yönetir. Özel kapatma davranışı: `TitleBar::new().on_close_window(|_, window, cx| …)`.

## 3. Tema erişimi

- `use gpui_kit::component::ActiveTheme as _;` ardından `cx.theme().background`, `cx.theme().primary`, …
- Bütün bir tema uygulamak: `Theme::global_mut(cx)` yuvaları + `Theme::change(mode, window, cx)` — `fluentui::theme::apply()` bunu yapar. Dağınık renk alanlarını elle değiştirmeyin.

## 4. Burada kullanılan bileşenler (içe aktarım yolları)

| Bileşen | İçe aktarım | Notlar |
|---|---|---|
| Button | `gpui_kit::component::button::{Button, ButtonVariants as _}` | `.label()`, varyantlar `.ghost()`, `.link()`, `.primary()`; `.tooltip("…")`, `.on_click(cx.listener(...))` |
| Checkbox | `gpui_kit::component::checkbox::Checkbox` | `.label().checked(bool).on_change(cx.listener(|this, v: &bool, _, cx| …))` |
| Switch | `gpui_kit::component::switch::Switch` | aynı on_change biçimi |
| RadioGroup | `gpui_kit::component::radio::RadioGroup` | `.children(["a","b"])` (`&str: Into<Radio>`), `.selected_index(Option<usize>)`, `.on_change(cx.listener(|this, ix: &usize, _, cx| …))` |
| Input | `gpui_kit::component::input::{Input, InputState}` | `let state = cx.new(|cx| InputState::new(window, cx).placeholder("…")); Input::new(&state)` |
| Select | `gpui_kit::component::select::{Select, SelectEvent, SelectState}` | delege = `Vec<SharedString>`; `SelectState::new(items, Some(IndexPath::new(0)), window, cx)`; `SelectEvent::Confirm(Option<SharedString>)` abone edilir |
| Slider | `gpui_kit::component::slider::{Slider, SliderEvent, SliderState}` | durum: `SliderState::new().min(0.).max(100.)`; `SliderEvent::Change(SliderValue)` → `value.start()` |
| Progress | `gpui_kit::component::progress::Progress` | `Progress::new(id).value(0.0..=1.0)` |
| Tooltip | düğmelerde `.tooltip("…")` | metin tabanlı |
| TitleBar | `gpui_kit::component::TitleBar` | `.child(div().child("…"))` |

## 5. Olaylar: `cx.listener` ve `cx.subscribe_in`

- Bileşen geri çağırları (`on_click`, `on_change`) `Fn(&E, &mut Window, &mut App)` alır — bunları `cx.listener(|this, event, window, cx| …)` ile üretin (`&mut ThemeGalleryApp` + `Context<Self>` verir; durum değişikliklerinden sonra `cx.notify()` çağırın).
- Varlık olayları (Select/Slider/Input): `new()` içinde bir kez abone olun ve `Subscription`'ı saklayın:

```rust
cx.subscribe_in(&channel, window, |this, _, event, _, cx| {
    if let SelectEvent::Confirm(Some(value)) = event { /* … */ cx.notify(); }
});
```

## 6. Yerleşim deyimleri

- Uygulama kabuğu: `div().flex().flex_col().size_full().bg(cx.theme().background).text_color(cx.theme().foreground)`; Tailwind tarzı boşluklar (`p_4`, `gap_2`, `size_4`, `w(px(220.))`).
- Fluent gövde boyutu için kökte `.text_size(px(14.))`.
- `TitleBar` yüksekliği sabittir (34px) — sayfa içeriğini kardeş bir `div().flex_1().min_h_0()` içine koyun.

## 7. Bu depoda görülen tuzaklar

- `Window`'ın kendine özgü bir `window_handle()`'ı var (`AnyWindowHandle` döner); ham X11 tanıtıcısı için `raw_window_handle::HasWindowHandle::window_handle(window)` çağrısını açıkça yapın.
- `ThemeConfig` içindeki `None`'lar gpui-kit varsayılanlarına sıfırlar — kısmi yapılandırmalar normdur.
- Wayland: protokol gereği pencere konumlama yoktur; CSD farklı davranır — X11 davranışını varsaymayın.
