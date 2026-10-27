# AppimageMannger: primeiros passos

AppimageMannger é um gerenciador nativo de aplicativos AppImage para Linux. Selecione um aplicativo na lista à esquerda para ver, à direita, ícone, versão, arquitetura, tamanho e localização. A importação copia o arquivo para a pasta gerenciada e mantém o download original.

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

## Licença e código-fonte

O projeto usa AGPL-3.0-only. Uso, alteração e redistribuição seguem essa licença. Distribuição e serviços de rede modificados exigem disponibilizar o código correspondente conforme os termos completos em LICENSE.

[Código e versões](https://github.com/xingwangzhe/AppimageMannger)
