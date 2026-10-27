# AppImage Yöneticisi

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage Yöneticisi, Linux için yerel bir AppImage uygulama yöneticisidir. Soldaki listeden bir uygulama seçerek sağda simgesini, sürümünü, mimarisini, boyutunu ve kurulum yolunu görebilirsiniz. İçe aktarma, dosyayı yönetilen klasöre kopyalar; özgün indirmeyi silmez. Ayrıntılardan uygulamayı başlatabilir ve masaüstü menü kaydını yönetebilirsiniz.

## Üç yerel arayüz

Tüm ekran görüntüleri 1.0.1 sürümünün İngilizce arayüzünü gösterir. Çevrilmiş README dosyaları aynı üç görüntüyü kullanır.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Özellikler

- İçe aktarma eski sürümün yerini alır. Özgün indirme korunur; önceki kurulumlar çöp kutusundan geri yüklenebilir.
- Uygulama simgeleri ve AppImage için “Birlikte aç” bütünleşmesi. `--repair-registration` kaydı onarır.
- Mimari ve SHA-256 denetimiyle anonim HTTPS üzerinden doğrudan Release indirme; gerçek ilerleme, yüzde ve aktarılan boyut. `gh` veya belirteç gerekmez.
- Ely/GPUI, Qt 6 Widgets ve GNOME/GTK 4 üzerinde sürüklenebilir ayırıcılar ve kalıcı tercihler; 26 dilde uygulama adı.
- AppImage dosyasını çalıştırmadan veya bağlamadan üst verileri ve simgeleri okur.

## Kurulum ve açılış

Releases bölümünden işlemcinize uygun paketi indirin ve aşağıdaki yöntemlerden birini seçin. Taşınabilir sürüm için AppimageMannger.AppImage adını gerçek dosya adıyla değiştirin. Arch paketleri sağlanır; bu rehber resmî bir AUR kaydı bulunduğunu varsaymaz.

```sh
# Debian / Ubuntu
sudo apt install ./appimage-mannger*.deb
# Fedora / RPM
sudo dnf install ./appimage-mannger*.rpm
# Arch Linux
sudo pacman -U ./appimage-mannger*.pkg.tar.zst
# AppImage
chmod +x ./AppimageMannger.AppImage
./AppimageMannger.AppImage
```

## Kalıcı ayarlar

Varsayılan klasör ~/Applications konumudur. Ayarlardan değiştirebilirsiniz; mevcut uygulamalar otomatik taşınmaz. Gerçek GPUI/Ely, Qt 6 Widgets veya GNOME/GTK 4/libadwaita arayüzünü seçin. Arayüz değişikliği tercihleri kaydeder ve yöneticiyi yeniden başlatır. Tema, klasör, dil, seçili uygulama ve pencere durumu korunur. Ayarlar → Dil bölümünde 26 dilden birini seçebilirsiniz. Error, Info ve Warning tanılama düzeyleri bilerek İngilizce bırakılmıştır.

## Güncelleme ve kaldırma

Güncelleme denetiminden önce uygulamanın GitHub deposunu doğrulayın. Desteklenen .upd_info verileri okunur; owner/repository elle de girilebilir. Kararlı bir sürüm arayın ve uygun AppImage dosyasını yükleyin. Boyut ve biçim denetlenir; özet yayımlanmışsa SHA-256 doğrulanır. Değiştirme atomiktir ve önceki sürüm yedeklenir. Başarısız güncelleme kurulu uygulamayı korur.

Kaldırma, uygulamayı ve masaüstü kayıtlarını çöp kutusuna taşır. Bunları geri yükleyebilirsiniz; kişisel ayarlarınız korunur.


## Geliştirme

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Lisans ve kaynak

Proje AGPL-3.0-only kullanır. Kullanım, değiştirme ve dağıtım bu lisansın koşullarına bağlıdır. Dağıtım ve değiştirilmiş ağ hizmetleri, ilgili kaynak kodunu sağlama yükümlülükleri doğurur. Tam koşullar için LICENSE dosyasını okuyun.

[Kaynak ve sürümler](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
