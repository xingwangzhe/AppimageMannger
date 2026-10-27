# AppImage प्रबंधक

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage प्रबंधक, Linux के लिए एक नेटिव AppImage प्रबंधक है। बाईं सूची में ऐप चुनें और दाईं ओर उसका आइकन, संस्करण, आर्किटेक्चर, आकार तथा इंस्टॉलेशन पथ देखें। आयात करने पर फ़ाइल प्रबंधित फ़ोल्डर में कॉपी होती है; मूल डाउनलोड हटता नहीं है। विवरण से ऐप चलाना और डेस्कटॉप मेनू प्रविष्टि सुधारना भी संभव है।

## तीन नेटिव इंटरफ़ेस

सभी चित्र संस्करण 1.0.1 का अंग्रेज़ी इंटरफ़ेस दिखाते हैं। हर अनूदित README में यही तीन चित्र हैं।

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## सुविधाएँ

- इंपोर्ट पुराना संस्करण बदल देता है। मूल डाउनलोड सुरक्षित रहता है और पिछली स्थापना कूड़ेदान से वापस लाई जा सकती है।
- ऐप आइकन और AppImage के लिए “इसके साथ खोलें” एकीकरण। `--repair-registration` पंजीकरण ठीक करता है।
- अनाम HTTPS से सीधे Release डाउनलोड, आर्किटेक्चर और SHA-256 जाँच, वास्तविक प्रगति, प्रतिशत और स्थानांतरित आकार। `gh` या टोकन की आवश्यकता नहीं है।
- Ely/GPUI, Qt 6 Widgets और GNOME/GTK 4 में खिसकने वाले विभाजक और सहेजी गई प्राथमिकताएँ; 26 भाषाओं में ऐप का नाम।
- AppImage को चलाए या माउंट किए बिना मेटाडेटा और आइकन पढ़ता है।

## इंस्टॉल करें और खोलें

Releases से अपने प्रोसेसर के लिए पैकेज डाउनलोड करें और नीचे का कोई एक तरीका चुनें। पोर्टेबल संस्करण के लिए AppimageMannger.AppImage को वास्तविक फ़ाइल नाम से बदलें। Arch पैकेज उपलब्ध हैं; यह मार्गदर्शिका किसी आधिकारिक AUR प्रविष्टि को नहीं मानती।

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

## सहेजी जाने वाली सेटिंग

डिफ़ॉल्ट फ़ोल्डर ~/Applications है। इसे सेटिंग में बदल सकते हैं; पुराने ऐप अपने आप स्थानांतरित नहीं होते। वास्तविक GPUI/Ely, Qt 6 Widgets या GNOME/GTK 4/libadwaita इंटरफ़ेस चुनें। बदलने पर प्राथमिकताएँ सहेजी जाती हैं और प्रबंधक फिर शुरू होता है। थीम, फ़ोल्डर, भाषा, चुना हुआ ऐप और विंडो की स्थिति सुरक्षित रहती है। सेटिंग → भाषा में 26 भाषाओं में से चुनें। निदान स्तर Error, Info और Warning जानबूझकर अंग्रेज़ी में रहते हैं।

## अपडेट और हटाना

अपडेट जाँचने से पहले ऐप का GitHub रिपॉज़िटरी सत्यापित करें। समर्थित .upd_info पढ़ा जाता है, या owner/repository स्वयं लिख सकते हैं। स्थिर रिलीज़ खोजकर सही आर्किटेक्चर का AppImage इंस्टॉल करें। आकार और फ़ॉर्मेट जाँचे जाते हैं; प्रकाशित डाइजेस्ट होने पर SHA-256 भी सत्यापित होता है। बदलाव परमाणु है और पुराने संस्करण का बैकअप रहता है। विफल अपडेट इंस्टॉल ऐप को सुरक्षित रखता है।

अनइंस्टॉल ऐप और डेस्कटॉप प्रविष्टियाँ कूड़ेदान में भेजता है, जहाँ से उन्हें वापस ला सकते हैं। व्यक्तिगत सेटिंग नहीं मिटती।


## विकास

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## लाइसेंस और स्रोत

परियोजना AGPL-3.0-only के अंतर्गत है। उपयोग, संशोधन और वितरण इसी लाइसेंस के अनुसार करें। वितरण और संशोधित नेटवर्क सेवाओं में संबंधित स्रोत उपलब्ध कराने की जिम्मेदारी है। पूरी शर्तें LICENSE में हैं।

[स्रोत और रिलीज़](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
