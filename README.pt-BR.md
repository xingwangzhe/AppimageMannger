# Gerenciador de AppImage

[![Linux CI](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml/badge.svg)](https://github.com/xingwangzhe/AppimageMannger/actions/workflows/linux.yml)

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Deutsch](README.de.md) · [Français](README.fr.md) · [Español](README.es.md) · [Português (Brasil)](README.pt-BR.md) · [Русский](README.ru.md) · [Italiano](README.it.md) · [Nederlands](README.nl.md) · [Polski](README.pl.md) · [Türkçe](README.tr.md) · [Українська](README.uk.md) · [Tiếng Việt](README.vi.md) · [Bahasa Indonesia](README.id.md) · [ไทย](README.th.md) · [العربية](README.ar.md) · [हिन्दी](README.hi.md) · [বাংলা](README.bn.md) · [فارسی](README.fa.md) · [עברית](README.he.md) · [Svenska](README.sv.md) · [Čeština](README.cs.md) · [Română](README.ro.md)

Gerenciador de AppImage é um gerenciador nativo de aplicativos AppImage para Linux. Selecione um aplicativo na lista à esquerda para ver, à direita, ícone, versão, arquitetura, tamanho e localização. A importação copia o arquivo para a pasta gerenciada e mantém o download original.

## Três interfaces nativas

Todas as capturas mostram a interface em inglês da versão 1.0.1. Cada README traduzido usa as mesmas três imagens.

### Ely / GPUI

![Ely / GPUI — English interface](docs/images/ely-gpui-en.png)

### Qt 6 Widgets

![Qt 6 Widgets — English interface](docs/images/qt-en.png)

### GNOME / GTK 4 / libadwaita

![GNOME / GTK 4 — English interface](docs/images/gnome-en.png)

## Recursos

- A importação substitui versões anteriores. O download original é preservado e instalações antigas podem ser restauradas da lixeira.
- Ícones e integração de AppImage em “Abrir com”. `--repair-registration` repara o registro.
- Downloads diretos de Releases por HTTPS anônimo, com verificação de arquitetura e SHA-256, progresso real, porcentagem e volume transferido. Não exige `gh` nem token.
- Divisórias ajustáveis e preferências salvas em Ely/GPUI, Qt 6 Widgets e GNOME/GTK 4; nomes de exibição em 26 idiomas.
- Lê metadados e ícones sem executar nem montar o AppImage.

## Instalar e abrir

Baixe em Releases um pacote compatível com seu processador e escolha um método abaixo. Para a versão portátil, substitua AppimageMannger.AppImage pelo nome real do arquivo. Há pacotes Arch; este guia não pressupõe uma publicação oficial no AUR.

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

## Configurações persistentes

A pasta padrão é ~/Applications. Altere-a nas configurações; os aplicativos existentes não são movidos automaticamente. Escolha uma interface real GPUI/Ely, Qt 6 Widgets ou GNOME/GTK 4/libadwaita. A troca salva as preferências e reinicia o gerenciador. Tema, pasta, idioma, aplicativo selecionado e estado da janela são preservados. Em Configurações → Idioma, escolha entre 26 idiomas. Os níveis de diagnóstico Error, Info e Warning permanecem intencionalmente em inglês.

## Atualizar e remover

Antes de verificar atualizações, confira o repositório GitHub do aplicativo. Os metadados .upd_info compatíveis são lidos, ou você pode informar owner/repository. Procure uma versão estável e instale o AppImage correspondente. Tamanho e formato são verificados; o SHA-256 também é validado quando um resumo é publicado. A substituição é atômica e mantém uma cópia da versão anterior. Uma falha preserva o aplicativo instalado.

A desinstalação move o aplicativo e seus atalhos para a lixeira. Você pode restaurá-los; suas configurações pessoais permanecem.


## Desenvolvimento

[BUILD-AND-RELEASE](docs/BUILD-AND-RELEASE.md) · [ARCHITECTURE](docs/ARCHITECTURE.md) · [i18n](docs/i18n/COVERAGE.md) · [1.0.2](docs/RELEASE-1.0.2.md) · [THIRD-PARTY-NOTICES](docs/THIRD-PARTY-NOTICES.md)

```sh
bun run scripts/check-i18n.ts
cargo fmt --all --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --release --locked
cargo build --release --locked
bun run scripts/package.ts --formats deb
```

## Licença e código-fonte

O projeto usa AGPL-3.0-only. Uso, alteração e redistribuição seguem essa licença. Distribuição e serviços de rede modificados exigem disponibilizar o código correspondente conforme os termos completos em LICENSE.

[Código e versões](https://github.com/xingwangzhe/AppimageMannger)

AGPL-3.0-only © 2026 [xingwangzhe](https://github.com/xingwangzhe) · [xingwangzhe@outlook.com](mailto:xingwangzhe@outlook.com) · [xingwangzhe.fun](https://xingwangzhe.fun) · [LICENSE](LICENSE)
