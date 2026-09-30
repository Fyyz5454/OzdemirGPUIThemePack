# PRD — OzdemirGPUIThemePack Ürün Gereksinimleri

Durum: yaşayan belge · Sürüm: ön yayın, depo sürümü **0.0.0** kalır · Son güncelleme: 2026-09-30

## 1. Ürün kimliği

**OzdemirGPUIThemePack**, [gpui-kit](https://gpui-kit.com/) için bir **Fluent UI tema paketidir**: Rust-öncelikli token kütüphanesi, CSS tema dosyaları ve canlı önizlemeli galeri uygulaması. gpui-kit üzerinde kurulan GPU çizimli uygulamalar, tek bir crate'e bağımlı olarak otantik Fluent 2 görünümü kazanır.

Proje geçmişi: depo 2026-09-30'da iptal edilen bir Linux dosya yöneticisi ürünü olarak başladı; eldeki Fluent tema altyapısı bu projeye dönüştü. Eski arka uç (saf-Rust motor yığını) karar gereği tamamen silindi.

## 2. Hedefler ve hedef dışılar

### Hedefler

- **Kütüphane crate'i `ozdemirgpuithemepack`**: Rust içinde Fluent 2 tasarım tokenleri (`fluentui::colors`), gpui-kit köprüsü (`fluentui::theme::apply`) ve CSS tema dosyası yükleyicisi (`css::parse`/`css::load`).
- **CSS tema dosyaları**: `themes/*.css` — varyant başına tek bir `:root { --key: value; }` sayfası, 8 vurgu rengi × açık/koyu. **JSON tema dosyası yok** (karar 1).
- **Galeri uygulaması `ThemeGallery`**: gpui-kit bileşen vitrini üzerinde 16 varyantın tümünde canlı geçiş; tema önizlemesi bir hata ayıklama aracı değil, ürünün kendisidir.
- Deterministik üretim: dosyalar Rust tokenlerinden `GenThemes` tarafından üretilir ve `themes_are_in_sync` testiyle bayt bayt doğrulanır.

### Hedef dışılar (açıkça hariç)

- Dosya yöneticisi, dosya işlemleri veya herhangi bir motor işi (iptal edilen ürün).
- JSON tema çıktısı veya gpui-kit `ThemeRegistry` dizin bağlantısı (karar 1'e bakın).
- Eklenti/uzantı sistemi.

## 3. Özellik seti

Kesinleşenler:

- 16 Fluent varyantı (Mavi, Kırmızı, Yeşil, Turkuaz, Mor, Macenta, Turuncu, Sarı × açık/koyu); doğru Fluent 2 nötr + vurgu rampalarıyla.
- Tipografi köprüsü: Segoe UI Variable aile zinciri çalışma zamanında çözülür, sessiz geri düşüşle.
- gpui-kit'in `Theme::change` yoluyla canlı tema uygulama (temel eşitleme, metin varsayılanları, pencere tazeleme).
- Katı dilbilgisine sahip CSS yükleyici: renkler `#RRGGBBAA`'ya normalleşir, bilinmeyen özellikler uyarır, eksik zorunlu özellikler hata verir.
- Galeri arayüzü (Türkçe dizgiler): vurgu radyo grubu, koyu tema anahtarı, varyant seçimi, varsayılana dönüş ve gpui-kit bileşen vitrini.

Yol haritası:

1. **Tema tasarımcısı** — görsel token düzenleyici (renk seçiciler, radius, yazı tipi), canlı önizleme, `css` modülü üzerinden `.css` tema dosyalarını aç/kaydet. Kararla ertelendi (2026-09-30); mimari `ozdemirgpuithemepack::css` üzerine kurulmalıdır, asla paralel bir biçim üzerine değil.
2. Kütüphane crate'inin crates.io yayını.
3. Olası ek tasarım dilleri (ör. Material) ayrı token modülleri olarak — geliştirme-zamanı varyantları, yayına göre kararlaştırılır.

## 4. Mimari

- `src/ozdemirgpuithemepack` — kütüphane. `src/fluentui/` (token doğruluk kaynağı: `colors.rs`, `typography.rs`, `fonts.rs`, `theme.rs`), `src/css.rs` (lightningcss tabanlı `.css` → `ThemeConfig` yükleyici), `src/bin/GenThemes.rs` (`themes/*.css` üretir).
- `src/ThemeGallery` — galeri binary'si; temaları yalnızca kütüphane üzerinden uygular.
- `themes/` — 16 üretilmiş CSS dosyası. **Üretilir, elle asla düzenlenmez.**
- Tema dosyası dilbilgisi: gpui-kit `ThemeConfigColors` şema anahtarı `primary.hover.background` → `--primary-hover-background`; meta anahtarlar `--name`, `--mode` (light|dark), `--radius`/`--radius-lg` (px), `--shadow` (none|drop); `font.family` bilinçli olarak yok (çalışma zamanında çözülür).

## 5. Kararlar (çözülenler)

1. **JSON yerine CSS — ÇÖZÜLDÜ (2026-09-30)**: gpui-kit'in `ThemeRegistry`'si yalnızca `*.json` okur (`registry.rs` JSON `ThemeSet`'leri tarar); yukarı akışta CSS desteği yoktur. Bağımlılığı yamalamak yerine paket kendi CSS giriş yolunu taşır (`ozdemirgpuithemepack::css`) ve JSON tema dosyalarını tamamen bırakır. `register_theme_dir` kaldırıldı. JSON çıktısının geri getirilmesi yeni bir karar ister.
2. **CSS çözümleyici: `lightningcss` — ÇÖZÜLDÜ (2026-09-30)**: kullanıcı, el yapımı tokenizer veya ince `cssparser` katmanı yerine hazır tam CSS çözümleyici tarafını seçti. Pinli `=1.0.0-alpha.72` (alpha kabul); tüm kullanım `src/css.rs` arkasında izole edilmiştir, çözümleyici çağıranları değiştirmeden takas edilebilir. Not: lightningcss renkleri normalleştirir (`#fff`, `rgba()`, hex sıkıştırma), bu yüzden yükleyici her rengi kanonik `#RRGGBBAA` olarak yeniden yazar.
3. **Ürün şekli: kütüphane + galeri (+ tasarımcı sonra) — ÇÖZÜLDÜ (2026-09-30)**.
4. **FM iptali — ÇÖZÜLDÜ (2026-09-30)**: commit'lenmemiş motor işi (fs/archive/thumbnail motor kodu, workspace motor bağımlılıkları) kullanıcı kararıyla arşiv dalı olmadan silindi.

## 6. Kısıtlar ve ortam notları

- `lightningcss` 1.0.0-alpha ve hareketli: pinli sürümü koruyun, her kullanımı `ozdemirgpuithemepack::css` arkasında tutun (MPL-2.0, saf Rust, Firefox/Servo'nun `cssparser`'ı üzerinde).
- Tasarımcı turu dosya biçimini netleştirene dek sürüm 0.0.0 kalır.
- X11: istemci tarafı dekorasyonlar çalışan compositor ister (`xprop -root _NET_WM_CM_S0`); yoksa gpui sunucu tarafı dekorasyonlara geri düşer. Wayland'da protokol gereği istemci tarafı pencere konumlandırma imkânsızdır.
- gpui kendi wgpu (blade) çizicisini gömer; ek GPU kurulumu gerekmez.

## 7. Açık kararlar (her biri ayrı bir soru-cevap turunda, aynı anda tek soru ile çözülür)

1. **Tema tasarımcısı MVP kapsamı** — hangi token grupları düzenlenebilir; `themes/` içine mi yoksa kullanıcı dizinine mi yazar?
2. **crates.io yayını** — zamanlama, isim müsaitliği, semver başlangıcı.
3. **Ek tasarım dilleri** — Material (veya başkaları) ikinci bir token modülü olacak mı?
