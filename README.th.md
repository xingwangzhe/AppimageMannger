# ตัวจัดการ AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

ตัวจัดการ AppImage เป็นตัวจัดการแอป AppImage แบบเนทีฟสำหรับ Linux เลือกแอปจากรายการด้านซ้ายเพื่อดูไอคอน รุ่น สถาปัตยกรรม ขนาด และตำแหน่งติดตั้งทางด้านขวา การนำเข้าจะคัดลอกไฟล์ไปยังโฟลเดอร์ที่จัดการ โดยไม่ลบไฟล์ดาวน์โหลดต้นฉบับ หน้ารายละเอียดยังใช้เปิดแอปและซ่อมรายการในเมนูเดสก์ท็อปได้

## อินเทอร์เฟซเนทีฟสามแบบ

ภาพทั้งหมดแสดงอินเทอร์เฟซภาษาอังกฤษของรุ่น 1.0.1 โดย README ทุกภาษาใช้ภาพสามภาพเดียวกันนี้

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## คุณสมบัติ

- การนำเข้าจะแทนที่รุ่นเก่า โดยเก็บไฟล์ดาวน์โหลดต้นฉบับไว้ และกู้คืนการติดตั้งเดิมจากถังขยะได้
- รองรับไอคอนแอปและเมนู “เปิดด้วย” สำหรับ AppImage ใช้ `--repair-registration` เพื่อซ่อมการลงทะเบียน
- ดาวน์โหลด Release โดยตรงผ่าน HTTPS แบบไม่ระบุตัวตน ตรวจสอบสถาปัตยกรรมและ SHA-256 พร้อมแสดงความคืบหน้า เปอร์เซ็นต์ และขนาดที่ดาวน์โหลดจริง ไม่ต้องมี `gh` หรือโทเคน
- ลากเส้นแบ่งและบันทึกค่ากำหนดใน Ely/GPUI, Qt 6 Widgets และ GNOME/GTK 4 พร้อมชื่อแอปที่แปลเป็น 26 ภาษา
- อ่านข้อมูลกำกับและไอคอนโดยไม่เรียกใช้หรือเมานต์ AppImage

## ติดตั้งและเปิดใช้งาน

ดาวน์โหลดแพ็กเกจที่ตรงกับหน่วยประมวลผลจาก Releases แล้วเลือกวิธีใดวิธีหนึ่งด้านล่าง สำหรับรุ่นพกพา ให้เปลี่ยน AppimageMannger.AppImage เป็นชื่อไฟล์ที่ดาวน์โหลดจริง มีแพ็กเกจ Arch ให้ใช้ แต่คู่มือนี้ไม่ได้ถือว่ามีรายการ AUR อย่างเป็นทางการแล้ว

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

## การตั้งค่าที่บันทึกไว้

โฟลเดอร์เริ่มต้นคือ ~/Applications เปลี่ยนได้ในการตั้งค่า โดยแอปเดิมจะไม่ย้ายให้อัตโนมัติ เลือกส่วนติดต่อจริงระหว่าง GPUI/Ely, Qt 6 Widgets และ GNOME/GTK 4/libadwaita เมื่อเปลี่ยน โปรแกรมจะบันทึกการตั้งค่าแล้วเริ่มตัวจัดการใหม่ ธีม โฟลเดอร์ ภาษา แอปที่เลือก และสถานะหน้าต่างจะคงอยู่ เลือกหนึ่งใน 26 ภาษาได้ที่ การตั้งค่า → ภาษา ระดับข้อความวินิจฉัย Error, Info และ Warning จงใจคงเป็นภาษาอังกฤษ

## อัปเดตและถอนการติดตั้ง

ก่อนตรวจสอบอัปเดต ให้ยืนยันคลัง GitHub ของแอป โปรแกรมอ่านข้อมูล .upd_info ที่รองรับ หรือให้กรอก owner/repository เองได้ ตรวจหารุ่นเสถียรแล้วติดตั้ง AppImage ที่ตรงกับสถาปัตยกรรม จะตรวจขนาดและรูปแบบไฟล์ และตรวจ SHA-256 หากผู้เผยแพร่ให้ค่าตรวจสอบมา การแทนที่เป็นแบบอะตอมมิกพร้อมสำรองรุ่นก่อนหน้า หากอัปเดตล้มเหลว แอปที่ติดตั้งไว้ยังคงอยู่

การถอนการติดตั้งจะย้ายแอปและรายการเดสก์ท็อปไปยังถังขยะเพื่อให้กู้คืนได้ การตั้งค่าส่วนตัวยังคงอยู่ การเปลี่ยนโฟลเดอร์ไม่ย้ายไฟล์เก่า


## การพัฒนา

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.1](docs/RELEASE-1.0.1.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## สัญญาอนุญาตและซอร์ส

โครงการใช้ AGPL-3.0-only อนุญาตให้ใช้ แก้ไข และแจกจ่ายตามเงื่อนไข การแจกจ่ายหรือให้บริการเครือข่ายด้วยรุ่นที่แก้ไขมีหน้าที่เผยแพร่ซอร์สที่เกี่ยวข้อง โปรดอ่าน LICENSE ฉบับเต็ม

[ซอร์สและรุ่นเผยแพร่](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 xingwangzhe · xingwangzhe@outlook.com · [LICENSE](LICENSE)
