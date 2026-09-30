# OzdemirGPUIThemePack

[gpui-kit](https://gpui-kit.com/) (Zed'in [GPUI](https://gpui.rs/) üzerindeki bileşen kütüphanesi) için **Fluent UI tema paketi**: Rust-öncelikli token kütüphanesi, CSS tema dosyaları ve canlı önizlemeli galeri uygulaması.

> Durum: erken aşama (sürüm `0.0.0` kalır). Geçmiş: bu depo iptal edilmiş bir dosya yöneticisi ürünü olarak başladı (2026-09-30'da iptal edildi) ve Fluent tema altyapısı proje oldu. Ürün tanımı: [PRD.md](PRD.md). Galeri arayüzü Türkçe önceliklidir.

## Ne elde edersiniz

- **Kütüphane crate'i `ozdemirgpuithemepack`** — Rust içinde Fluent 2 tokenleri, gpui-kit tema köprüsü ve CSS tema dosyası yükleyicisi:

  ```rust
  use ozdemirgpuithemepack::fluentui::{theme, Accent};

  // Doğrudan Rust tokenlerinden:
  theme::apply(Accent::Teal, true, Some(window), cx);

  // Ya da üretilmiş bir tema dosyasından:
  use ozdemirgpuithemepack::css;
  let config = css::load(std::path::Path::new("themes/FluentDarkTeal.css"))?;
  theme::apply_config(config, Some(window), cx);
  ```

- **16 tema dosyası** (`themes/*.css`, 8 vurgu rengi × açık/koyu), her biri tek `:root` custom-property sayfası:

  ```css
  /* OzdemirGPUIThemePack — FluentLightBlue */
  :root {
    --name: "FluentLightBlue";
    --mode: light;
    --radius: 4px;
    --radius-lg: 8px;
    --shadow: none;
    --background: #f3f3f3ff;
    --foreground: #000000e4;
    --primary-background: #0066b4ff;
    --primary-hover-background: #0078d4ff;
    /* … */
  }
  ```

  gpui-kit şema anahtarı (`primary.hover.background`), `--primary-hover-background` ile eşleşir; renkler her zaman `#RRGGBBAA`'ya normalleşir. gpui-kit'in kendi `ThemeRegistry`'si yalnızca JSON okur — CSS yolunu bu crate sağlar (`ozdemirgpuithemepack::css`).

- **Galeri uygulaması `ThemeGallery`** — tüm gpui-kit bileşenleri aktif varyant altında; canlı vurgu/açık-koyu geçişiyle (tema önizlemesi **ürünün kendisidir**).

## Depo yapısı

```
src/ozdemirgpuithemepack/   Kütüphane crate'i (tokenler, gpui-kit köprüsü, CSS yükleyici, GenThemes bin)
  src/fluentui/             Fluent 2 tasarım tokenleri → gpui-kit (Rust öncelikli)
  src/css.rs                lightningcss .css → ThemeConfig yükleyici
src/ThemeGallery/           Galeri uygulaması (canlı tema önizlemesi)
themes/                     Üretilmiş 16 CSS tema dosyası (8 renk × açık/koyu)
.kilo/skills/               Proje yetenekleri (FluentTheme, GpuiKit)
tr/                         Belgelerin Türkçe karşılıkları
```

## Gereksinimler

- X11'li Linux (Wayland: çizim çalışır; aşağıdaki notlara bakın)
- Rust (en son kararlı sürüm)
- Sistem paketleri (gpui):

  ```sh
  sudo apt install libfontconfig1-dev libxkbcommon-dev libxkbcommon-x11-dev libxcb1-dev
  ```

## Derleme ve çalıştırma

```sh
cargo build --workspace          # her şeyi derle
./run.sh                         # derle + galeriyi çalıştır
cargo run -p ThemeGallery        # galeriyi doğrudan çalıştır
cargo test --workspace           # testler (themes_are_in_sync dahil)
cargo clippy --workspace --all-targets
```

### Fluent tema dosyaları

`themes/*.css` Rust token modülünden **üretilir** — asla elle düzenlemeyin. `src/ozdemirgpuithemepack/src/fluentui/colors.rs` veya `theme.rs` değiştirildikten sonra:

```sh
cargo run -p ozdemirgpuithemepack --bin GenThemes
```

CSS dosyaları Rust kaynağından ayrışırsa `themes_are_in_sync` testi başarısız olur.

## Pencere ve başlık çubuğu

Galeri, gpui-kit'in istemci tarafı `TitleBar`'ını `WindowDecorations::Client` ile kullanır; sistem (WM) başlık çubuğu, uygulama içi Fluent başlık çubuğuyla değiştirilir. X11'de bu, çalışan bir compositor ister; yoksa GPUI otomatik olarak sunucu tarafı dekorasyonlara geri döner (AGENTS.md → Ortam notları).

## Belgeler

- [PRD.md](PRD.md) — ürün gereksinimleri (tema paketi, açık kararlar)
- [AGENTS.md](AGENTS.md) — katılımcı/ajan rehberi (derleme kuralları, uzlaşmalar)
- [tr/](.) — bu belgelerin Türkçe karşılıkları
- Yetenekler: [FluentTheme](../.kilo/skills/FluentTheme/SKILL.md), [GpuiKit](../.kilo/skills/GpuiKit/SKILL.md)

## Yol haritası (açık kararlar)

1. Tema tasarımcısı — canlı önizlemeli görsel token düzenleyici, `ozdemirgpuithemepack::css` üzerinde
2. Kütüphane crate'inin crates.io yayını
3. Ek tasarım dilleri (ör. Material), geliştirme-zamanı varyantları olarak

## Ekran görüntüleri

_Yok_

## Lisans

Proje lisansı TBD. CSS çözümleyici (`lightningcss`) MPL-2.0'dır; bu depoya GPL kod girmez.
