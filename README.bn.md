# AppImage ব্যবস্থাপক

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage ব্যবস্থাপক হলো Linux-এর জন্য একটি নেটিভ AppImage ব্যবস্থাপক। বাঁ পাশের তালিকা থেকে অ্যাপ নির্বাচন করলে ডানে তার আইকন, সংস্করণ, আর্কিটেকচার, আকার ও ইনস্টলেশনের পথ দেখা যায়। আমদানি করলে ফাইলটি পরিচালিত ফোল্ডারে কপি হয়; মূল ডাউনলোড মুছে যায় না। বিস্তারিত অংশ থেকে অ্যাপ চালানো এবং ডেস্কটপ মেনুর এন্ট্রি ঠিক করাও সম্ভব।

## তিনটি নেটিভ ইন্টারফেস

সব ছবিতে 1.0.1 সংস্করণের ইংরেজি ইন্টারফেস দেখানো হয়েছে। প্রতিটি অনূদিত README একই তিনটি ছবি ব্যবহার করে।

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## বৈশিষ্ট্য

- আমদানি পুরোনো সংস্করণ প্রতিস্থাপন করে। মূল ডাউনলোড রাখা হয় এবং আগের ইনস্টলেশন ট্র্যাশ থেকে ফেরানো যায়।
- অ্যাপের আইকন ও AppImage-এর “অন্য অ্যাপে খুলুন” সংযোগ। `--repair-registration` নিবন্ধন মেরামত করে।
- বেনামি HTTPS দিয়ে সরাসরি Release ডাউনলোড, আর্কিটেকচার ও SHA-256 পরীক্ষা, প্রকৃত অগ্রগতি, শতাংশ ও স্থানান্তরিত আকার। `gh` বা টোকেন লাগে না।
- Ely/GPUI, Qt 6 Widgets ও GNOME/GTK 4-এ টেনে সরানো বিভাজক ও সংরক্ষিত পছন্দ; ২৬ ভাষায় প্রদর্শিত অ্যাপের নাম।
- AppImage চালানো বা মাউন্ট করা ছাড়াই মেটাডেটা ও আইকন পড়ে।

## ইনস্টল ও চালু করা

Releases থেকে আপনার প্রসেসরের উপযুক্ত প্যাকেজ নামিয়ে নিচের একটি পদ্ধতি বেছে নিন। বহনযোগ্য সংস্করণের জন্য AppimageMannger.AppImage-এর বদলে প্রকৃত ফাইলের নাম দিন। Arch প্যাকেজ দেওয়া হয়; এই নির্দেশিকা কোনো সরকারি AUR এন্ট্রি আছে বলে ধরে নেয় না।

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

## সংরক্ষিত সেটিংস

ডিফল্ট ফোল্ডার ~/Applications। সেটিংস থেকে এটি বদলানো যায়; পুরোনো অ্যাপ স্বয়ংক্রিয়ভাবে সরানো হয় না। প্রকৃত GPUI/Ely, Qt 6 Widgets অথবা GNOME/GTK 4/libadwaita ইন্টারফেস নির্বাচন করুন। পরিবর্তনে পছন্দ সংরক্ষণ করে ব্যবস্থাপক আবার চালু হয়। থিম, ফোল্ডার, ভাষা, নির্বাচিত অ্যাপ ও উইন্ডোর অবস্থা থেকে যায়। সেটিংস → ভাষা থেকে ২৬টি ভাষার একটি বেছে নিন। Error, Info ও Warning নির্ণয় স্তর ইচ্ছাকৃতভাবে ইংরেজিতে থাকে।

## আপডেট ও অপসারণ

আপডেট পরীক্ষার আগে অ্যাপের GitHub রিপোজিটরি যাচাই করুন। সমর্থিত .upd_info তথ্য পড়া হয়; চাইলে owner/repository নিজে লিখতে পারেন। স্থিতিশীল সংস্করণ খুঁজে সঠিক আর্কিটেকচারের AppImage ইনস্টল করুন। আকার ও বিন্যাস পরীক্ষা হয়; প্রকাশিত ডাইজেস্ট থাকলে SHA-256 যাচাই হয়। প্রতিস্থাপন পারমাণবিক এবং পুরোনো সংস্করণের ব্যাকআপ থাকে। আপডেট ব্যর্থ হলে ইনস্টল করা অ্যাপ অক্ষত থাকে।

আনইনস্টল অ্যাপ ও ডেস্কটপ এন্ট্রি ট্র্যাশে পাঠায়, সেখান থেকে ফিরিয়ে আনা যায়। ব্যক্তিগত সেটিংস মুছে যায় না।


## উন্নয়ন

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.1](docs/RELEASE-1.0.1.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## লাইসেন্স ও সোর্স

প্রকল্পটি AGPL-3.0-only ব্যবহার করে। ব্যবহার, পরিবর্তন ও বিতরণ এই লাইসেন্স মেনে করা যায়। বিতরণ বা পরিবর্তিত নেটওয়ার্ক সেবায় সংশ্লিষ্ট সোর্স দেওয়ার বাধ্যবাধকতা আছে। সম্পূর্ণ শর্ত LICENSE-এ দেখুন।

[সোর্স ও রিলিজ](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com · [LICENSE](LICENSE)
