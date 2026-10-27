# AppImage 관리자

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

AppImage 관리자는 Linux용 네이티브 AppImage 관리자입니다. 왼쪽 목록에서 앱을 선택하면 오른쪽에 아이콘, 버전, 아키텍처, 크기와 설치 경로가 표시됩니다. 가져오기는 관리 폴더에 파일을 복사하는 작업이며 원래 다운로드 파일을 삭제하지 않습니다. 상세 화면에서 실행하거나 폴더와 데스크톱 메뉴 항목을 관리할 수 있습니다.

## 세 가지 네이티브 인터페이스

모든 스크린샷은 1.0.1 버전의 영어 인터페이스입니다. 각 언어 README는 동일한 세 이미지를 사용합니다.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## 주요 기능

- 새 버전을 가져오면 기존 버전을 교체합니다. 원본 다운로드는 유지하며 이전 버전은 휴지통에서 복원할 수 있습니다.
- 앱 아이콘과 AppImage의 연결 프로그램을 등록합니다. `--repair-registration`으로 등록을 복구할 수 있습니다.
- 익명 HTTPS로 Release를 직접 다운로드하고 아키텍처와 SHA-256을 확인합니다. 실제 진행률과 전송량을 표시하며 `gh`나 토큰이 필요 없습니다.
- Ely/GPUI, Qt 6 Widgets, GNOME/GTK 4에서 구분선을 드래그하고 설정을 저장합니다. 앱 표시 이름은 26개 언어를 지원합니다.
- AppImage를 실행하거나 마운트하지 않고 메타데이터와 아이콘을 읽습니다.

## 설치하고 실행하기

Releases에서 프로세서에 맞는 패키지를 받은 뒤 아래 방법 중 하나를 선택하세요. 휴대용 버전은 AppimageMannger.AppImage를 실제 다운로드 파일 이름으로 바꿉니다. Arch 패키지가 제공되지만 이 문서는 공식 AUR 등록을 전제로 하지 않습니다.

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

## 유지되는 설정

기본 폴더는 ~/Applications입니다. 설정에서 변경해도 기존 앱이 자동으로 이동하지는 않습니다. 실제 GPUI/Ely, Qt 6 Widgets 또는 GNOME/GTK 4/libadwaita 프런트엔드를 선택할 수 있습니다. 백엔드를 변경하면 설정을 저장한 후 관리자를 다시 시작합니다. 테마, 폴더, 언어, 선택한 앱과 창 상태가 유지됩니다. ‘설정 → 언어’에서 26개 언어 중 하나를 선택하세요. 진단 수준 Error, Info, Warning은 의도적으로 영어로 유지됩니다.

## 업데이트와 제거

업데이트를 확인하기 전에 해당 앱의 GitHub 저장소를 확인하세요. 지원되는 .upd_info를 읽거나 owner/repository를 직접 입력할 수 있습니다. 안정 릴리스를 확인한 뒤 아키텍처에 맞는 AppImage를 설치합니다. 다운로드의 크기와 형식을 확인하고 공개된 다이제스트가 있으면 SHA-256도 검증합니다. 교체는 원자적으로 수행하며 이전 버전을 백업합니다. 업데이트가 실패하면 설치된 앱이 유지됩니다.

제거하면 앱과 데스크톱 항목을 휴지통으로 옮기므로 복원할 수 있습니다. 개인 설정은 삭제하지 않습니다.


## 개발

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.3](docs/RELEASE-1.0.3.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## 라이선스와 소스

AGPL-3.0-only에 따라 사용, 수정, 재배포할 수 있습니다. 배포하거나 수정된 네트워크 서비스를 제공할 때 해당 소스를 제공할 의무가 있습니다. 전체 조건은 LICENSE를 확인하세요.

[소스 및 릴리스](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
